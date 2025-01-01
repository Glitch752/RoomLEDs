use crate::{render::frame::Frame, TOTAL_PIXELS};

use super::Compositor;

/// An alpha compositor composites layers together using alpha blending.
pub struct AlphaCompositor;

impl Compositor for AlphaCompositor {
    fn composite(&self, layers: Vec<Frame>) -> Frame {
        let mut final_frame = Frame::empty();

        for layer in layers {
            for i in 0..TOTAL_PIXELS {
                let pixel = &layer.pixel_data[i as usize];
                let final_pixel = &mut final_frame.pixel_data[i as usize];

                let alpha = pixel.alpha;
                let inv_alpha = 1.0 - alpha;

                final_pixel.r = (final_pixel.r as f64 * inv_alpha + pixel.r as f64 * alpha) as u8;
                final_pixel.g = (final_pixel.g as f64 * inv_alpha + pixel.g as f64 * alpha) as u8;
                final_pixel.b = (final_pixel.b as f64 * inv_alpha + pixel.b as f64 * alpha) as u8;
                final_pixel.alpha = final_pixel.alpha.max(pixel.alpha);
            }
        }

        final_frame
    }
}