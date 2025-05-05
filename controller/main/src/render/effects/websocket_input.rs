use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame, RenderInfo};

use super::{AnyEffect, Effect, RenderContext};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct WebsocketInputEffect {}

impl WebsocketInputEffect {
    /// Creates a new websocket input effect.
    #[allow(unused)]
    pub fn new() -> AnyEffect {
        Self {}.into()
    }
}

impl Effect for WebsocketInputEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> frame::Frame {
        let mut frame: frame::Frame = frame::Frame::empty(context.pixels);
        if render_info.websocket_input.is_none() {
            return frame;
        }
        let websocket_input = render_info.websocket_input.as_ref().unwrap();

        for (i, pixel) in websocket_input.chunks(3).enumerate() {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            frame.set_pixel(i as u32, frame::PixelColor { r, g, b, alpha: 1. });
        }

        frame
    }
}