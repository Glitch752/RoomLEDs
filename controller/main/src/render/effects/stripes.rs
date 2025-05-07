use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::{Frame, PixelColor}, RenderInfo};

use super::{AnyEffect, Effect, RenderContext};

/// The stripes effect renders a pattern with stripes of color. To make the stripes move, a rotate effect may be used.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct StripeEffect {
    /// The width of each stripe in pixels.
    stripe_width: f64,
    /// The colors of the stripes.
    stripe_colors: Vec<PixelColor>
}

impl StripeEffect {
    /// Creates a new stripes effect with the specified stripe width and colors.
    #[allow(unused)]
    pub fn new(stripe_width: f64, stripe_colors: Vec<PixelColor>) -> AnyEffect {
        StripeEffect {
            stripe_width,
            stripe_colors
        }.into()
    }
}

impl Effect for StripeEffect {
    fn render(&mut self, context: RenderContext, _render_info: &mut RenderInfo) -> Frame {
        let mut frame = Frame::empty(context.pixels);

        for i in 0..context.pixels {
            let stripe_pos = i as f64;

            let stripe_index = (stripe_pos / self.stripe_width).floor() as usize % self.stripe_colors.len();
            let stripe_color = &self.stripe_colors[stripe_index];
            let rgb = color_space::Rgb::new(
                stripe_color.r as f64,
                stripe_color.g as f64,
                stripe_color.b as f64,
            );

            let fade = 1. - (stripe_pos % self.stripe_width) / self.stripe_width;

            let mut pixel: PixelColor = rgb.into();
            pixel.alpha = fade * stripe_color.alpha;

            frame.set_pixel(i, pixel);
        }

        frame
    }
}