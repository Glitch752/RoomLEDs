use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{render::frame::Frame, RenderInfo, TOTAL_PIXELS};

use super::{Effect, AnyEffect};

// TODO: Deduplicate the compositor code with a macro

/// An additive compositor composites other effects together using additive blending.
#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export)]
pub struct AdditiveCompositorEffect {
    effects: Vec<Box<AnyEffect>>
}

impl AdditiveCompositorEffect {
    /// Creates a new additive compositor effect with the specified effects.
    /// Returns a boxed effect.
    pub fn new(effects: Vec<Box<AnyEffect>>) -> Box<AnyEffect> {
        Box::new(AdditiveCompositorEffect {
            effects
        }.into())
    }
}

impl Effect for AdditiveCompositorEffect {
    fn render(&mut self, delta: std::time::Duration, render_info: &mut RenderInfo) -> Frame {
        let mut final_frame = Frame::empty();

        for effect in &mut self.effects {
            let rendered_frame = effect.render(delta, render_info);

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