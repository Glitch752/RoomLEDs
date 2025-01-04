use crate::{render::frame::Frame, RenderState, TOTAL_PIXELS};

use super::Effect;

pub struct RotateEffect {
    /// The effect to rotate
    effect: Box<dyn Effect>,
    /// The number of pixels to rotate the frame by
    rotation: i32
}

impl RotateEffect {
    /// Creates a new 
    /// Returns a boxed effect.
    pub fn new(effect: Box<dyn Effect>, rotation: i32) -> Box<RotateEffect> {
        Box::new(RotateEffect {
            effect,
            rotation
        })
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