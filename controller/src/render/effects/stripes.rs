use std::time::Duration;

use crate::{render::frame::{Frame, Pixel}, RenderState, TOTAL_PIXELS};

use super::Effect;

/// The stripes effect renders a rotating pattern with stripes of color.
pub struct StripeEffect {
    stripe_width: f64,
    stripe_colors: Vec<(u8, u8, u8)>,
    speed: f64,
}

impl StripeEffect {
    /// Creates a new stripes effect with the specified stripe width, colors, and speed.
    /// Returns a boxed effect.
    pub fn new(stripe_width: f64, stripe_colors: Vec<(u8, u8, u8)>, speed: f64) -> Box<StripeEffect> {
        Box::new(StripeEffect {
            stripe_width,
            stripe_colors,
            speed,
        })
    }
}

impl Effect for StripeEffect {
    fn render(&mut self, _delta: Duration, render_state: &RenderState) -> Frame {
        let mut frame = Frame::empty();

        for i in 0..TOTAL_PIXELS {
            let stripe_pos = (i as f64 + render_state.time * self.speed).round();

            let stripe_index = (stripe_pos / self.stripe_width).floor() as usize % self.stripe_colors.len();
            let rgb = color_space::Rgb::new(
                self.stripe_colors[stripe_index].0 as f64,
                self.stripe_colors[stripe_index].1 as f64,
                self.stripe_colors[stripe_index].2 as f64,
            );

            let fade = 1. - (stripe_pos % self.stripe_width) / self.stripe_width;

            let mut pixel: Pixel = rgb.into();
            pixel.alpha = fade;

            frame.set_pixel(i, pixel);
        }

        frame
    }
}