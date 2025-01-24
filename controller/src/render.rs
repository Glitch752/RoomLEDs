use std::{sync::Arc, thread::JoinHandle, time::Duration};

use filters::Filter;
use frame::PresentedFrame;
use parking_lot::Mutex;
use ringbuf::{traits::{Observer, Producer, Split}, StaticRb};
use thread_priority::{ThreadBuilderExt, ThreadPriority, ThreadPriorityValue};

use crate::{RenderState, FRAME_TIMES_STORED};

pub mod effects;
mod filters;
pub mod spatial_map;

pub mod frame;
mod idle_tracker;

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
            let (info, effect) = state.split();

            info.time += delta.as_secs_f64();
            
            info.frames += 1;

            let frames = info.frames;
            info.frame_times[frames % FRAME_TIMES_STORED] = delta.as_secs_f64();

            // Render the effect
            let effect_frame = effect.render(delta, info);

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
                idle_tracker.update(&frame);

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