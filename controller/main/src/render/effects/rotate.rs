use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::Frame, RenderInfo, TOTAL_PIXELS};

use super::{AnyEffect, Effect};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct RotateEffect {
    /// The effect to rotate
    effect: Box<AnyEffect>,
    /// The number of pixels to rotate the frame by. If negative, it rotates to the left.
    rotation: i32
}

impl RotateEffect {
    /// Creates a new rotate effect.
    #[allow(unused)]
    pub fn new(effect: Box<AnyEffect>, rotation: i32) -> AnyEffect {
        RotateEffect {
            effect,
            rotation
        }.into()
    }
}

impl Effect for RotateEffect {
    fn render(&mut self, delta: std::time::Duration, render_info: &mut RenderInfo) -> Frame {
        let rendered_frame = self.effect.render(delta, render_info);
        
        let mut rotated_frame = Frame::empty();
        for i in 0..TOTAL_PIXELS {
            let new_i = (i as i32 + self.rotation).rem_euclid(TOTAL_PIXELS as i32) as usize;
            rotated_frame.set_pixel(new_i as u32, rendered_frame.get_pixel(i));
        }

        rotated_frame
    }
}