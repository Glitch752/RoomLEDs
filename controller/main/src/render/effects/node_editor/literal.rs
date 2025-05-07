use reflection::Reflect;
use serde::{Deserialize, Serialize};

use super::{NodeImplementation, Value};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct LiteralNode {
    pub value: Value
}

impl NodeImplementation for LiteralNode {
    fn should_rerender(&self) -> bool {
        return false;
    }
}