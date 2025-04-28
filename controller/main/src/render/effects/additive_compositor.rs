use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::Frame, RenderInfo, TOTAL_PIXELS};

use super::{Effect, AnyEffect};

// TODO: Deduplicate the compositor code with a macro

/// An additive compositor composites other effects together using additive blending.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct AdditiveCompositorEffect {
    /// The effects to be composited together. The output of every effect will be added together.
    effects: Vec<Box<AnyEffect>>
}

impl AdditiveCompositorEffect {
    /// Creates a new additive compositor effect with the specified effects.
    #[allow(unused)]
    pub fn new(effects: Vec<Box<AnyEffect>>) -> AnyEffect {
        AdditiveCompositorEffect {
            effects
        }.into()
    }
}

impl Effect for AdditiveCompositorEffect {
    fn render(&mut self, delta: std::time::Duration, render_info: &mut RenderInfo) -> Frame {
        let mut final_frame = Frame::empty();

        for effect in &mut self.effects {
            let rendered_frame = effect.render(delta, render_info);

            for i in 0..TOTAL_PIXELS {
                let pixel = rendered_frame.get_pixel(i);
                let final_pixel = match final_frame.get_pixel_mut(i) {
                    Some(pixel) => pixel,
                    None => { continue; }
                };

                final_pixel.r = final_pixel.r.saturating_add(pixel.r);
                final_pixel.g = final_pixel.g.saturating_add(pixel.g);
                final_pixel.b = final_pixel.b.saturating_add(pixel.b);
                final_pixel.alpha = final_pixel.alpha.max(pixel.alpha);
            }
        }

        final_frame
    }
}