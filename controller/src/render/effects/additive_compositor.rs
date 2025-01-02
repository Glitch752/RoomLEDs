use crate::{render::frame::Frame, TOTAL_PIXELS};

use super::Effect;

// TODO: Deduplicate the compositor code with a macro

/// An additive compositor composites other effects together using additive blending.
pub struct AdditiveCompositorEffect {
    effects: Vec<Box<dyn Effect>>
}

impl AdditiveCompositorEffect {
    /// Creates a new additive compositor effect with the specified effects.
    /// Returns a boxed effect.
    pub fn new(effects: Vec<Box<dyn Effect>>) -> Box<AdditiveCompositorEffect> {
        Box::new(AdditiveCompositorEffect {
            effects
        })
    }
}

impl Effect for AdditiveCompositorEffect {
    fn render(&mut self, delta: std::time::Duration, render_state: &crate::RenderState) -> Frame {
        let mut final_frame = Frame::empty();

        for effect in &mut self.effects {
            let rendered_frame = effect.render(delta, render_state);

            for i in 0..TOTAL_PIXELS {
                let pixel = rendered_frame.get_pixel(i);
                let final_pixel = final_frame.get_pixel_mut(i);

                final_pixel.r = final_pixel.r.saturating_add(pixel.r);
                final_pixel.g = final_pixel.g.saturating_add(pixel.g);
                final_pixel.b = final_pixel.b.saturating_add(pixel.b);
                final_pixel.alpha = final_pixel.alpha.max(pixel.alpha);
            }
        }

        final_frame
    }
}