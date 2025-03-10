use crate::Reflect;
use serde::{Deserialize, Serialize};

// Namespace ourself as "reflection"
pub mod reflection {
    pub use crate::*;
}

#[derive(Reflect, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Schema {
    Struct(Vec<SchemaField>),
    Enum(Vec<EnumVariant>),
    /** A reference to another schema definition. Used to avoid infinite recursion. */
    Reference(String),
    Number,
    String,
    Boolean
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub ty: Schema
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<Schema>
}