use std::{sync::Arc, thread::JoinHandle, time::Duration};

use effects::{AnyEffect, TemporaryEffectCompositor};
use filters::Filter;
use frame::PresentedFrame;
use parking_lot::Mutex;
use ringbuf::{traits::{Observer, Producer, Split}, StaticRb};
use spatial_map::Location;
use thread_priority::{ThreadBuilderExt, ThreadPriority, ThreadPriorityValue};

use crate::{FRAME_TIMES_STORED, TOTAL_PIXELS};

pub mod effects;
mod filters;
pub mod spatial_map;
pub mod frame;
mod idle_tracker;
// State for rendering the lights that needs to be shared between the web server and the output thread
#[derive(Debug)]
pub struct RenderState {
    pub info: RenderInfo,
    pub temporary_effect_compositor: TemporaryEffectCompositor,
    pub effect: Box<AnyEffect>
}

impl RenderState {
    fn split(&mut self) -> (&mut RenderInfo, &mut TemporaryEffectCompositor, &mut dyn effects::Effect) {
        (&mut self.info, &mut self.temporary_effect_compositor, self.effect.as_mut())
    }
}

#[derive(Debug)]
pub struct RenderInfo {
    // The time in seconds since rendering started
    pub time: f64,

    // Statistics we collect to display on the web interface
    // We can't use a dynamic array here because allocating in the output thread is not allowed
    pub frame_times: [f64; FRAME_TIMES_STORED],
    pub frames: usize,
    
    pub current_presented_frame: Option<PresentedFrame>,
    pub debug_text: String,
    pub pixel_locations: [Location; TOTAL_PIXELS as usize],
    pub websocket_input: Option<Vec<u8>>
}

impl RenderInfo {
    pub fn new(pixel_locations: [Location; TOTAL_PIXELS as usize]) -> Self {
        Self {
            time: 0.0,
            frame_times: [0.0; FRAME_TIMES_STORED],
            frames: 0,
            current_presented_frame: None,
            debug_text: "".to_string(),
            pixel_locations,
            websocket_input: None
        }
    }
}

/// The number of frames we render ahead of time. We use this to avoid
/// dropping frames if the render thread runs slightly behind for a frame.
static RENDER_BUFFER_SIZE: usize = 2;

type RenderRingBuf = StaticRb::<PresentedFrame, RENDER_BUFFER_SIZE>;
pub type RenderRingBufConsumer = <RenderRingBuf as Split>::Cons;
type RenderRingBufProducer = <RenderRingBuf as Split>::Prod;

pub fn render_frame(delta: Duration, render_state: &Arc<Mutex<RenderState>>, filters: &Vec<Box<dyn Filter>>) -> Option<PresentedFrame> {
    // We should never hold a lock on the render state for a significant amount of time in other threads
    match render_state.try_lock_for(Duration::from_millis(1)) {
        Some(mut state) => {
            let (info, temporary_effect_compositor, effect) = state.split();

            info.time += delta.as_secs_f64();
            
            info.frames += 1;

            let frames = info.frames;
            info.frame_times[frames % FRAME_TIMES_STORED] = delta.as_secs_f64();

            // Render the effect
            let effect_frame = effects::AlphaCompositorEffect::composite(vec![
                effect,
                temporary_effect_compositor
            ], delta, info);

            let mut presented_frame: PresentedFrame = effect_frame.into();

            // We store the frame before applying filters so we can display it in the UI
            // before filtering. Filters are used to correct the colors of the frame, which
            // just makes colors look worse in the UI.
            info.current_presented_frame = Some(presented_frame.clone());

            // Apply filters
            for filter in filters {
                presented_frame = filter.apply(presented_frame);
            }

            Some(presented_frame)
        }
        None => {
            eprintln!("Warning: failed to lock render state after 1ms. This caused a dropped frame.");
            None
        }
    }
}

fn run_render_thread(render_state: Arc<Mutex<RenderState>>, mut producer: RenderRingBufProducer) {
    let mut last_frame_time = std::time::Instant::now();
    
    let filters: Vec<Box<dyn Filter>> = vec![
        filters::GammaCorrectionFilter::new(2.2)
    ];

    let mut idle_tracker = idle_tracker::IdleTracker::new(
        Duration::from_secs(5 * 60),
        Duration::from_secs(0),
        Box::new(idle_tracker::esphome_plug::ESPHomePlug::new(
            "192.168.68.131".to_string(),
            "kauf_plug".to_string(),
            "kauf_plug_power".to_string(),
        ))
    );

    loop {
        loop {
            let start_time = std::time::Instant::now();
            let delta = start_time - last_frame_time;
            last_frame_time = start_time;
    
            if let Some(frame) = render_frame(delta, &render_state, &filters) {
                // TEMPORARY/HACK: If the hostname isn't "lighting", don't update the idle tracker
                if std::env::var("HOSTNAME").unwrap_or("".to_string()) == "lighting" {
                    idle_tracker.update(&frame);
                }

                // It's possible that we continue looping but the ring buffer is full in
                // some edge cases. In that case, we just drop the frame.
                _ = producer.try_push(frame);
            }

            if producer.is_full() {
                break;
            }
        }

        // Wait for the ring buffer to have space for the next frame.
        // The output thread will wake us up when we can render another frame.
        std::thread::park();
    }
}

pub fn start_render_thread(render_state: Arc<Mutex<RenderState>>) -> (JoinHandle<()>, RenderRingBufConsumer) {
    let rb = RenderRingBuf::default();
    let (producer, consumer) = rb.split();

    (
        std::thread::Builder::new()
            .name("lightingOutputThread".to_string())
            .spawn_with_priority(ThreadPriority::Crossplatform(ThreadPriorityValue::try_from(90).unwrap()), |result| {
                match result {
                    Ok(_) => println!("Successfully started render thread!"),
                    Err(e) => {
                        eprintln!("Failed to start render thread with a higher priority: {:?}", e);
                    }
                };
                
                run_render_thread(render_state, producer);
            })
            .expect("Failed to create output thread"),
        consumer
    )
}