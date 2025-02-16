use std::time::Duration;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{render::frame, RenderInfo};

use super::{AnyEffect, Effect};

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export)]
pub struct WebsocketInputEffect {}

impl WebsocketInputEffect {
    /// Creates a new websocket input effect.
    /// Returns a boxed effect.
    #[allow(unused)]
    pub fn new() -> Box<AnyEffect> {
        Box::new(Self {}.into())
    }
}

impl Effect for WebsocketInputEffect {
    fn render(&mut self, _delta: Duration, render_info: &mut RenderInfo) -> frame::Frame {
        let mut frame: frame::Frame = frame::Frame::empty();
        if render_info.websocket_input.is_none() {
            return frame;
        }
        let websocket_input = render_info.websocket_input.as_ref().unwrap();

        for (i, pixel) in websocket_input.chunks(3).enumerate() {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            frame.set_pixel(i as u32, frame::Pixel { r, g, b, alpha: 1. });
        }

        frame
    }
}