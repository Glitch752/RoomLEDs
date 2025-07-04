use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::{effects::{AnyEffect, AnyTemporaryEffect, Effect, RenderContext, TemporaryEffect}, frame::{self}}, RenderInfo};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct DurationTemporaryEffect {
    // The effect duration in seconds
    duration: f64,
    // The time the effect started
    #[serde(skip)]
    start_time: f64,
    // The effect to apply
    effect: Box<AnyEffect>
}

impl DurationTemporaryEffect {
    /// Creates a new temporary effect with the specified duration and effect.
    /// Returns a boxed temporary effect.
    #[allow(unused)]
    pub fn new(duration: f64, effect: Box<AnyEffect>) -> Box<AnyTemporaryEffect> {
        Box::new(Self {
            duration,
            start_time: 0.0,
            effect
        }.into())
    }
}

impl Effect for DurationTemporaryEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> frame::Frame {
        self.effect.render(context, render_info)
    }
}

impl TemporaryEffect for DurationTemporaryEffect {
    fn start(&mut self, render_info: &mut RenderInfo) {
        self.start_time = render_info.time;
    }

    fn is_finished(&self, render_info: &RenderInfo) -> bool {
        render_info.time - self.start_time >= self.duration
    }

    fn stop(&mut self, _render_info: &mut RenderInfo) {}
}