use std::{collections::HashMap, fmt::Debug};
use node::{Node, NodeID};
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

#[derive(Clone)]
struct NodeData {
    instance: Box<dyn Node>
}

impl Reflect for NodeData {
    fn ts_definition() -> String {
        todo!()
    }
    
    fn schema() -> reflection::schema::Schema {
        todo!()
    }
    
    fn visit_dependencies(_: &mut impl reflection::TypeVisitor) where Self: 'static {
        todo!()
    }
    // TODO...
}

impl Serialize for NodeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO...
        Ok(())
    }
}

impl Deserialize<'_> for NodeData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO...
        Ok(NodeData { instance: Box::new(nodes::SimpleNode {}) })
    }
}

impl Debug for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeData")
            .field("instance", &self.instance.name())
            .finish()
    }
}

/// An effect that renders a frame based on a node-based graphical editor.
/// This is by far the most complex effect type, as it allows for arbitrary
/// calculations for every pixel in the frame.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct NodeEditorEffect {
    nodes: HashMap<NodeID, NodeData>
}

impl Effect for NodeEditorEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        // TODO
        Frame::empty(context.pixels)
    }
}