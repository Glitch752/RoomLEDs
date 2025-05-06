use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::{expressions::{AnyExpression, Expression}, frame::Frame}, RenderInfo};

use super::{AnyEffect, Effect, RenderContext};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct RotateEffect {
    /// The effect to rotate
    effect: Box<AnyEffect>,
    /// The number of pixels to rotate the frame by. If negative, it rotates to the left.
    rotation: Box<AnyExpression>
}

impl RotateEffect {
    /// Creates a new rotate effect.
    #[allow(unused)]
    pub fn new(effect: Box<AnyEffect>, rotation: Box<AnyExpression>) -> AnyEffect {
        RotateEffect {
            effect,
            rotation
        }.into()
    }
}

impl Effect for RotateEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        let rendered_frame = self.effect.render(context, render_info);
        
        let mut rotated_frame = Frame::empty(context.pixels);
        let rot = self.rotation.compute(&context.expression_context());
        for i in 0..context.pixels {
            let new_i = (i as i32 + rot as i32).rem_euclid(context.pixels as i32) as usize;
            rotated_frame.set_pixel(new_i as u32, rendered_frame.get_pixel(i));
        }

        rotated_frame
    }
}