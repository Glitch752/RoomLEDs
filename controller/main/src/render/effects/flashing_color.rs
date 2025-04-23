use std::{f64::consts::PI, time::Duration};

use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::{self, PixelColor}, RenderInfo, TOTAL_PIXELS};

use super::{AnyEffect, Effect};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct FlashingColorEffect {
    #[serde(skip)]
    time: f64,
    /// The offset of the flashing effect, in seconds.
    offset: f64,
    /// The speed of the flashing effect, in Hz.
    speed: f64,
    /// The color of the first flashing color.
    color_a: PixelColor,
    /// The color of the second flashing color.
    color_b: PixelColor
}

impl FlashingColorEffect {
    /// Creates a new flashing color effect with the specified speed and colors.
    /// Speed is in Hz.
    #[allow(unused)]
    pub fn new(speed: f64, offset: f64, color_a: PixelColor, color_b: PixelColor) -> AnyEffect {
        Self {
            time: 0., offset, speed, color_a, color_b
        }.into()
    }
}

impl Effect for FlashingColorEffect {
    fn render(&mut self, delta: Duration, _render_info: &mut RenderInfo) -> frame::Frame {
        self.time += delta.as_secs_f64();

        let mut frame: frame::Frame = frame::Frame::empty();

        let color = self.color_a.lerp(&self.color_b, (self.time * self.speed * 2. * PI).sin() * 0.5 + 0.5);

        for pixel in 0..TOTAL_PIXELS {
            frame.set_pixel(pixel, color.clone());
        }

        frame
    }
}