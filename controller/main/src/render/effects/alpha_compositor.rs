use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::Frame, RenderInfo};

use super::{AnyEffect, Effect, RenderContext};

/// An alpha compositor composites other effects together using alpha blending.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct AlphaCompositorEffect {
    /// The effects to be composited together. The output of every effect will be composited based on their order and transparency.
    effects: Vec<Box<AnyEffect>>
}

impl AlphaCompositorEffect {
    /// Creates a new alpha compositor effect with the specified effects.
    #[allow(unused)]
    pub fn new(effects: Vec<Box<AnyEffect>>) -> AnyEffect {
        AlphaCompositorEffect {
            effects
        }.into()
    }

    pub fn composite(effects: Vec<&mut dyn Effect>, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
       let mut final_frame = Frame::empty(context.pixels);

        for effect in effects {
            let rendered_frame = effect.render(context, render_info);

            for i in 0..context.pixels {
                let pixel = rendered_frame.get_pixel(i);
                let final_pixel = match final_frame.get_pixel_mut(i) {
                    Some(pixel) => pixel,
                    None => { continue; }
                };

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

impl Effect for AlphaCompositorEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        let effects = self.effects
            .iter_mut()
            .map(|effect| effect.as_mut() as &mut dyn Effect)
            .collect::<Vec<_>>();
        AlphaCompositorEffect::composite(effects, context, render_info)
    }
}