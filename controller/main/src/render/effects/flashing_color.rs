use std::time::Duration;

use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::{self, PixelColor}, RenderInfo, TOTAL_PIXELS};

use super::{AnyEffect, Effect};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct FlashingColorEffect {
    time: f64,
    speed: f64,
    color: PixelColor
}

impl FlashingColorEffect {
    /// Creates a new flashing color effect with the specified speed and color.
    /// Speed is in Hz.
    #[allow(unused)]
    pub fn new(speed: f64, color: PixelColor) -> AnyEffect {
        Self {
            time: 0., speed, color
        }.into()
    }
}

impl Effect for FlashingColorEffect {
    fn render(&mut self, delta: Duration, _render_info: &mut RenderInfo) -> frame::Frame {
        self.time += delta.as_secs_f64();

        let mut frame: frame::Frame = frame::Frame::empty();

        let red = self.color.with_alpha((self.time * self.speed).sin() * 0.5 + 0.5);

        for pixel in 0..TOTAL_PIXELS {
            frame.set_pixel(pixel, red.clone());
        }

        frame
    }
}