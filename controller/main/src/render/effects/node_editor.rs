use std::{fmt::Debug};
use reflection::Reflect;
use serde::{Deserialize, Serialize};
use types::{TypeInfo};
use crate::{render::frame::Frame, RenderInfo};
use super::{Effect, RenderContext};

#[macro_use]
mod types;
mod nodes;
mod node;
#[macro_use]
mod registry;

/// An effect that renders a frame based on a node-based graphical editor.
/// This is by far the most complex effect type, as it allows for arbitrary
/// calculations for every pixel in the frame.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct NodeEditorEffect {
    node_editor_name: String
}

impl Effect for NodeEditorEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        // TODO
        Frame::empty(context.pixels)
    }
}