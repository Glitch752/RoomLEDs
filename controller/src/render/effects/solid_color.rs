use std::time::Duration;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{render::frame::{self, Pixel}, RenderInfo};

use super::{AnyEffect, Effect};

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export)]
pub struct SolidColorEffect {
    color: Pixel,
    start: u32,
    stop: u32,
}

impl SolidColorEffect {
    /// Creates a new solid color effect with the specified color, start, and stop.
    /// Returns a boxed effect.
    #[allow(unused)]
    pub fn new(color: Pixel, start: u32, stop: u32) -> Box<AnyEffect> {
        Box::new(Self {
            color, start, stop
        }.into())
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