use std::{cell::RefCell, collections::HashMap, sync::LazyLock};
use parking_lot::Mutex;
use super::node::Node;

// Rust moment
pub static NODE_REGISTRY: LazyLock<Mutex<RefCell<NodeRegistry>>> = LazyLock::new(|| {
    Mutex::new(RefCell::new(NodeRegistry::new()))
});

#[macro_export]
macro_rules! register_node {
    ($name:expr, $node:expr) => {
        paste::paste! {
            #[ctor::ctor]
            #[allow(non_snake_case)]
            fn [<register_node_ $name>]() {
                use crate::render::effects::node_editor::registry::NODE_REGISTRY;
                NODE_REGISTRY.lock().borrow_mut().register_node(
                    $name,
                    Box::new($node),
                );
            }
        }
    };
}

pub struct NodeRegistry {
    nodes: HashMap<String, Box<dyn Node + Send + Sync>>
}

impl NodeRegistry {
    fn new() -> Self {
        Self {
            nodes: HashMap::new() 
        }
    }

    pub fn register_node(&mut self, name: &str, node: Box<dyn Node + Send + Sync>) {
        self.nodes.insert(name.to_string(), node);
    }

    pub fn get_node(&self, name: &str) -> Option<&Box<dyn Node + Send + Sync>> {
        self.nodes.get(name)
    }
}

