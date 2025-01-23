use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{render::frame::Frame, RenderState, TOTAL_PIXELS};

use super::{AnyEffect, Effect};

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
pub struct RotateEffect {
    /// The effect to rotate
    effect: Box<AnyEffect>,
    /// The number of pixels to rotate the frame by
    rotation: i32
}

impl RotateEffect {
    /// Creates a new 
    /// Returns a boxed effect.
    pub fn new(effect: Box<AnyEffect>, rotation: i32) -> Box<AnyEffect> {
        Box::new(RotateEffect {
            effect,
            rotation
        }.into())
    }
}

impl Effect for RotateEffect {
    fn render(&mut self, delta: std::time::Duration, render_state: &mut RenderState) -> Frame {
        let rendered_frame = self.effect.render(delta, render_state);
        
        let mut rotated_frame = Frame::empty();
        for i in 0..TOTAL_PIXELS {
            let new_i = (i as i32 + self.rotation).rem_euclid(TOTAL_PIXELS as i32) as usize;
            rotated_frame.set_pixel(new_i as u32, rendered_frame.get_pixel(i));
        }

        rotated_frame
    }
}