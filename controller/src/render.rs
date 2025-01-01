use std::{sync::Arc, thread::JoinHandle, time::Duration};

use parking_lot::Mutex;
use ringbuf::{traits::{Observer, Producer, Split}, StaticRb};
use thread_priority::{ThreadBuilderExt, ThreadPriority, ThreadPriorityValue};

use crate::{RenderState, FRAME_TIMES_STORED, TOTAL_PIXELS};

static GAMMA: f64 = 2.2;

#[derive(Debug, Clone)]
pub struct PresentedFrame {
    pub pixel_data: [u8; TOTAL_PIXELS as usize * 3]
}

/// The number of frames we render ahead of time. We use this to avoid
/// dropping frames if the render thread runs slightly behind for a frame.
static RENDER_BUFFER_SIZE: usize = 2;

type RenderRingBuf = StaticRb::<PresentedFrame, RENDER_BUFFER_SIZE>;
pub type RenderRingBufConsumer = <RenderRingBuf as Split>::Cons;
type RenderRingBufProducer = <RenderRingBuf as Split>::Prod;

pub fn render_frame(delta: Duration, render_state: &Arc<Mutex<RenderState>>) -> Option<PresentedFrame> {
    // We should never hold a lock on the render state for a significant amount of time in other threads
    match render_state.try_lock_for(Duration::from_millis(1)) {
        Some(mut state) => {
            state.start_hue += delta.as_secs_f64() * 240.;
            
            let mut pixel_data = [0; TOTAL_PIXELS as usize * 3];

            stripe_pattern(&mut pixel_data, state.start_hue);
            apply_gamma_correction(&mut pixel_data);

            state.frames += 1;

            let frames = state.frames;
            state.frame_times[frames % FRAME_TIMES_STORED] = delta.as_secs_f64();

            let presented_frame = PresentedFrame {
                pixel_data
            };

            state.current_presented_frame = Some(presented_frame.clone());
            Some(presented_frame)
        }
        None => {
            eprintln!("Warning: failed to lock render state after 1ms. This caused a dropped frame.");
            None
        }
    }
}

fn stripe_pattern(pixel_data: &mut [u8; TOTAL_PIXELS as usize * 3], start_hue: f64) {
    for i in 0..TOTAL_PIXELS {
        // let hue = (state.start_hue + (i as f64) * 6.) % 360.;
        // let color = color_space::Hsl::new(hue, 1., 0.5);
        // let rgb = color.to_rgb();
        // let rgb = color_space::Rgb::new(255., 255., 255.);

        // Stripe pattern
        static STRIPE_WIDTH: f64 = TOTAL_PIXELS  as f64 / 28.;
        static STRIPE_COLORS: [(u8, u8, u8); 7] = [
            (255, 0, 0),
            (255, 100, 0),
            (255, 255, 0),
            (0, 255, 0),
            (0, 0, 255),
            (143, 0, 255),
            (255, 255, 255),
        ];

        let stripe_pos = (i as f64 + start_hue / 2.).round();

        let stripe_index = (stripe_pos / STRIPE_WIDTH).floor() as usize % STRIPE_COLORS.len();
        let rgb = color_space::Rgb::new(
            STRIPE_COLORS[stripe_index].0 as f64,
            STRIPE_COLORS[stripe_index].1 as f64,
            STRIPE_COLORS[stripe_index].2 as f64,
        );

        let fade = 1. - (stripe_pos  % STRIPE_WIDTH) / STRIPE_WIDTH;
        let rgb = color_space::Rgb::new(
            rgb.r * fade,
            rgb.g * fade,
            rgb.b * fade,
        );

        pixel_data[(i * 3 + 0) as usize] = rgb.r as u8;
        pixel_data[(i * 3 + 1) as usize] = rgb.g as u8;
        pixel_data[(i * 3 + 2) as usize] = rgb.b as u8;
    };
}

fn apply_gamma_correction(pixel_data: &mut [u8; TOTAL_PIXELS as usize * 3]) {
    for i in 0..TOTAL_PIXELS {
        let r = pixel_data[(i * 3 + 0) as usize] as f64 / 255.;
        let g = pixel_data[(i * 3 + 1) as usize] as f64 / 255.;
        let b = pixel_data[(i * 3 + 2) as usize] as f64 / 255.;

        let r = r.powf(GAMMA);
        let g = g.powf(GAMMA);
        let b = b.powf(GAMMA);

        pixel_data[(i * 3 + 0) as usize] = (r * 255.) as u8;
        pixel_data[(i * 3 + 1) as usize] = (g * 255.) as u8;
        pixel_data[(i * 3 + 2) as usize] = (b * 255.) as u8;
    }
}

fn run_render_thread(render_state: Arc<Mutex<RenderState>>, mut producer: RenderRingBufProducer) {
    let mut last_frame_time = std::time::Instant::now();

    loop {
        loop {
            let start_time = std::time::Instant::now();
            let delta = start_time - last_frame_time;
            last_frame_time = start_time;
    
            if let Some(frame) = render_frame(delta, &render_state) {
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