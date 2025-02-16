use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{render::frame::Frame, RenderInfo, TOTAL_PIXELS};

use super::{AnyEffect, Effect};

/// An alpha compositor composites other effects together using alpha blending.
#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export)]
pub struct AlphaCompositorEffect {
    effects: Vec<Box<AnyEffect>>
}

impl AlphaCompositorEffect {
    /// Creates a new alpha compositor effect with the specified effects.
    /// Returns a boxed effect.
    #[allow(unused)]
    pub fn new(effects: Vec<Box<AnyEffect>>) -> Box<AnyEffect> {
        Box::new(AlphaCompositorEffect {
            effects
        }.into())
    }

    pub fn composite(effects: Vec<&mut dyn Effect>, delta: std::time::Duration, render_info: &mut RenderInfo) -> Frame {
       let mut final_frame = Frame::empty();

        for effect in effects {
            let rendered_frame = effect.render(delta, render_info);

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

impl Effect for AlphaCompositorEffect {
    fn render(&mut self, delta: std::time::Duration, render_info: &mut RenderInfo) -> Frame {
        let effects = self.effects
            .iter_mut()
            .map(|effect| effect.as_mut() as &mut dyn Effect)
            .collect::<Vec<_>>();
        AlphaCompositorEffect::composite(effects, delta, render_info)
    }
}