use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::{Frame, PixelColor}, RenderInfo};
use super::{Effect, RenderContext};

mod literal;

pub use literal::LiteralNode;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
struct NodeID(uuid::Uuid);

impl NodeID {
    pub fn new() -> NodeID {
        NodeID(uuid::Uuid::new_v4())
    }
}

impl Reflect for NodeID {
    const INLINE: bool = true;
    fn ts_definition() -> String {
        return "string".to_string();
    }
    fn schema() -> reflection::schema::Schema {
        reflection::schema::Schema::String
    }
    fn visit_dependencies(_: &mut impl reflection::TypeVisitor) where Self: 'static {}
}

enum ValueType {
    Float,
    Integer,
    Color,
    Boolean,
    Frame
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum Value {
    Float(f32),
    Integer(i32),
    Color(PixelColor),
    Boolean(bool),
    Frame(Frame),
}

#[enum_dispatch]
pub trait NodeImplementation {
    fn should_rerender(&self) -> bool {
        return true;
    }
}

#[enum_dispatch(NodeImplementation)]
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum AnyNodeImplementation {
    LiteralNode(LiteralNode)
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    id: NodeID,
    implementation: AnyNodeImplementation,
    inputs: Vec<(NodeID, usize)>,
    last_frame_rendered: u32,
    #[serde(skip)]
    output_values: Vec<Value>
}

impl Node {
    pub fn new(implementation: AnyNodeImplementation) -> Self {
        Self {
            id: NodeID::new(),
            implementation,
            inputs: Vec::new(),
            last_frame_rendered: 0,
            output_values: Vec::new(),
        }
    }

    pub fn get_output(&self, index: usize) -> Option<&Value> {
        self.output_values.get(index)
    }
}

/// An effect that renders a frame based on a node-based graphical editor.
/// This is by far the most complex effect type, as it allows for arbitrary
/// calculations for every pixel in the frame.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct NodeEditorEffect {
    nodes: HashMap<NodeID, Node>
}

impl Effect for NodeEditorEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        // TODO
        Frame::empty(context.pixels)
    }
}