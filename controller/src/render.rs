use std::{sync::Arc, thread::JoinHandle, time::Duration};

use compositors::Compositor;
use filters::Filter;
use frame::PresentedFrame;
use layers::Layer;
use parking_lot::Mutex;
use ringbuf::{traits::{Observer, Producer, Split}, StaticRb};
use thread_priority::{ThreadBuilderExt, ThreadPriority, ThreadPriorityValue};

use crate::{RenderState, FRAME_TIMES_STORED};

mod layers;
mod filters;
mod compositors;
pub mod frame;

/// The number of frames we render ahead of time. We use this to avoid
/// dropping frames if the render thread runs slightly behind for a frame.
static RENDER_BUFFER_SIZE: usize = 2;

type RenderRingBuf = StaticRb::<PresentedFrame, RENDER_BUFFER_SIZE>;
pub type RenderRingBufConsumer = <RenderRingBuf as Split>::Cons;
type RenderRingBufProducer = <RenderRingBuf as Split>::Prod;

type RenderConstructs = (Vec<Box<dyn Layer>>, Box<dyn Compositor>, Vec<Box<dyn Filter>>);

pub fn render_frame(delta: Duration, render_state: &Arc<Mutex<RenderState>>, render_constructs: &mut RenderConstructs) -> Option<PresentedFrame> {
    let (layers, compositor, filters) = render_constructs;

    // We should never hold a lock on the render state for a significant amount of time in other threads
    match render_state.try_lock_for(Duration::from_millis(1)) {
        Some(mut state) => {
            state.time += delta.as_secs_f64();
            
            state.frames += 1;

            let frames = state.frames;
            state.frame_times[frames % FRAME_TIMES_STORED] = delta.as_secs_f64();


            // Render the layers
            let rendered_layers = layers.iter_mut()
                .map(|layer| layer.render(&*state))
                .collect::<Vec<_>>();

            // Compose the layers
            let composed_frame = compositor.composite(rendered_layers);

            // Apply filters
            let mut final_frame: PresentedFrame = composed_frame.into();
            for filter in filters {
                final_frame = filter.apply(final_frame);
            }

            state.current_presented_frame = Some(final_frame.clone());
            Some(final_frame)
        }
        None => {
            eprintln!("Warning: failed to lock render state after 1ms. This caused a dropped frame.");
            None
        }
    }
}

fn run_render_thread(render_state: Arc<Mutex<RenderState>>, mut producer: RenderRingBufProducer) {
    let mut last_frame_time = std::time::Instant::now();

    // This is a temporary setup; I want to create a better builder pattern for this
    let compositor: Box<dyn Compositor> = Box::new(compositors::AdditiveCompositor);
    let layers: Vec<Box<dyn Layer>> = vec![
        // layers::StripeLayer::new(TOTAL_PIXELS  as f64 / 28., vec![
        //     (255, 0, 0),
        //     (255, 100, 0),
        //     (255, 255, 0),
        //     (0, 255, 0),
        //     (0, 0, 255),
        //     (143, 0, 255),
        //     (255, 255, 255),
        // ], 86.0),
        Box::new(layers::MusicVisualizerLayer::new(3001))
    ];
    let filters: Vec<Box<dyn Filter>> = vec![
        Box::new(filters::GammaCorrectionFilter::new(2.2))
    ];

    let mut render_constructs: RenderConstructs = (layers, compositor, filters);

    loop {
        loop {
            let start_time = std::time::Instant::now();
            let delta = start_time - last_frame_time;
            last_frame_time = start_time;
    
            if let Some(frame) = render_frame(delta, &render_state, &mut render_constructs) {
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