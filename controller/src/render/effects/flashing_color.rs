use std::time::Duration;

use crate::{render::frame::{self, Pixel}, RenderState, TOTAL_PIXELS};

use super::Effect;


pub struct FlashingColorEffect {
    time: f64,
    speed: f64,
    color: Pixel
}

impl FlashingColorEffect {
    /// Creates a new flashing color effect with the specified speed and color.
    /// Speed is in Hz.
    pub fn new(speed: f64, color: Pixel) -> Box<Self> {
        Box::new(Self {
            time: 0., speed, color
        })
    }
}

impl Effect for FlashingColorEffect {
    fn render(&mut self, delta: Duration, _render_state: &mut RenderState) -> frame::Frame {
        self.time += delta.as_secs_f64();

        let mut frame: frame::Frame = frame::Frame::empty();

        let red = self.color.with_alpha((self.time * self.speed).sin() * 0.5 + 0.5);

        for pixel in 0..TOTAL_PIXELS {
            frame.set_pixel(pixel, red.clone());
        }

        frame
    }
}