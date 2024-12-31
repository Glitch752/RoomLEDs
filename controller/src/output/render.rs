use std::{sync::Arc, time::Duration};

use parking_lot::Mutex;

use crate::RenderState;

use super::TOTAL_PIXELS;

pub fn render_frame(pixel_data: &mut Vec<u8>, delta: Duration, render_state: &Arc<Mutex<RenderState>>) {
    // We should never hold a lock on the render state for a significant amount of time in other threads
    match render_state.try_lock_for(Duration::from_millis(1)) {
        Some(mut state) => {
            state.start_hue += delta.as_secs_f64() * 240.;
            
            for i in 0..TOTAL_PIXELS {
                // let hue = (state.start_hue + (i as f64) * 6.) % 360.;
                // let color = color_space::Hsl::new(hue, 1., 0.5);
                // let rgb = color.to_rgb();
                // // let rgb = color_space::Rgb::new(255., 255., 255.);

                // Stripe pattern
                static STRIPE_WIDTH: f64 = 25.;
                static STRIPE_COLORS: [(u8, u8, u8); 6] = [
                    (255, 0, 0),
                    (255, 100, 0),
                    (255, 255, 0),
                    (0, 255, 0),
                    (0, 0, 255),
                    (143, 0, 255),
                ];

                let stripe_pos = (i as f64 + state.start_hue / 3.).round();

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
        None => {
            eprintln!("Warning: failed to lock render state after 1ms. This caused a dropped frame.");
        }
    };
}