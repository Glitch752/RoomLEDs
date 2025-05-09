use reflection::Reflect;
use serde::{Deserialize, Serialize};

use super::{NodeImplementation, Value};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct AddNode {
    pub value: Value
}

impl NodeImplementation for AddNode {
}