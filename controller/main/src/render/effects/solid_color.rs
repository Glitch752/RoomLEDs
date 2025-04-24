use std::time::Duration;

use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::{self, PixelColor}, RenderInfo};

use super::{AnyEffect, Effect};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct SolidColorEffect {
    /// The color to make every pixel.
    color: PixelColor,
    /// The start pixel index.
    start: u32,
    /// The stop pixel index.
    stop: u32,
}

impl SolidColorEffect {
    /// Creates a new solid color effect with the specified color, start, and stop.
    #[allow(unused)]
    pub fn new(color: PixelColor, start: u32, stop: u32) -> AnyEffect {
        Self {
            color, start, stop
        }.into()
    }
}

impl Effect for SolidColorEffect {
    fn render(&mut self, _delta: Duration, _render_info: &mut RenderInfo) -> frame::Frame {
        let mut frame: frame::Frame = frame::Frame::empty();

        for i in self.start..self.stop {
            frame.set_pixel(i, self.color.clone());
        }

        frame
    }
}