use std::time::{Duration};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{render::{effects::{AnyEffect, Effect, TemporaryEffect}, frame::{self}}, RenderInfo};

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export)]
pub struct DurationTemporaryEffect {
    duration: Duration,
    start_time: f64,
    effect: AnyEffect
}

impl DurationTemporaryEffect {
}

impl Effect for DurationTemporaryEffect {
    fn render(&mut self, delta: Duration, render_info: &mut RenderInfo) -> frame::Frame {
        self.effect.render(delta, render_info)
    }
}

impl TemporaryEffect for DurationTemporaryEffect {
    fn start(&mut self, render_info: &mut RenderInfo) {
        self.start_time = render_info.time;
    }

    fn is_finished(&self, render_info: &RenderInfo) -> bool {
        self.start_time >= self.duration.as_secs_f64()
    }

    fn stop(&mut self, _render_info: &mut RenderInfo) {
    }
}