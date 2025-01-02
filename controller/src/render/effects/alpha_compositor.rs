use crate::{render::frame::Frame, TOTAL_PIXELS};

use super::Effect;

/// An alpha compositor composites other effects together using alpha blending.
pub struct AlphaCompositorEffect {
    effects: Vec<Box<dyn Effect>>
}

impl AlphaCompositorEffect {
    /// Creates a new alpha compositor effect with the specified effects.
    /// Returns a boxed effect.
    pub fn new(effects: Vec<Box<dyn Effect>>) -> Box<AlphaCompositorEffect> {
        Box::new(AlphaCompositorEffect {
            effects
        })
    }
}

impl Effect for AlphaCompositorEffect {
    fn render(&mut self, delta: std::time::Duration, render_state: &crate::RenderState) -> Frame {
        let mut final_frame = Frame::empty();

        for effect in &mut self.effects {
            let rendered_frame = effect.render(delta, render_state);

            for i in 0..TOTAL_PIXELS {
                let pixel = rendered_frame.get_pixel(i);
                let final_pixel = final_frame.get_pixel_mut(i);

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