use crate::{render::frame::Frame, TOTAL_PIXELS};

use super::Compositor;

/// An additive compositor composites layers together using additive blending.
pub struct AdditiveCompositor;

impl Compositor for AdditiveCompositor {
    fn composite(&self, layers: Vec<Frame>) -> Frame {
        let mut final_frame = Frame::empty();

        for layer in layers {
            for i in 0..TOTAL_PIXELS {
                let pixel = &layer.pixel_data[i as usize];
                let final_pixel = &mut final_frame.pixel_data[i as usize];

                final_pixel.r = final_pixel.r.saturating_add(pixel.r);
                final_pixel.g = final_pixel.g.saturating_add(pixel.g);
                final_pixel.b = final_pixel.b.saturating_add(pixel.b);
                final_pixel.alpha = final_pixel.alpha.max(pixel.alpha);
            }
        }

        final_frame
    }
}