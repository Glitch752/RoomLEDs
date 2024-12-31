use std::{sync::Arc, time::Duration};

use color_space::ToRgb;
use parking_lot::Mutex;

use crate::RenderState;

use super::TOTAL_PIXELS;

pub fn render_frame(pixel_data: &mut Vec<u8>, delta: Duration, render_state: &Arc<Mutex<RenderState>>) {
    // We should never hold a lock on the render state for a significant amount of time in other threads
    match render_state.try_lock_for(Duration::from_millis(1)) {
        Some(mut state) => {
            state.start_hue += delta.as_secs_f64() * 240.;
            
            for i in 0..TOTAL_PIXELS {
                let hue = (state.start_hue + (i as f64) * 6.) % 360.;
                let color = color_space::Hsl::new(hue, 1., 0.5);
                let rgb = color.to_rgb();
                // let rgb = color_space::Rgb::new(255., 255., 255.);
        
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