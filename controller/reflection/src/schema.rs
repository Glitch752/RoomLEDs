use crate::Reflect;
use serde::{Deserialize, Serialize};

// Namespace ourself as "reflection" so we can use our own proc-macro.
// This probably isn't the best way to do it? It works for now.
mod reflection {
    pub use crate::*;
}

/// A schema definition for a type.
#[derive(Reflect, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
#[reflect(export_runtime_schema)]
pub enum Schema {
    Struct(Vec<SchemaField>),
    Enum(EnumValue),
    Optional(Box<Schema>),
    ArrayOf(Box<Schema>),
    TupleOf(Vec<Schema>),
    ObjectOf(ObjectSchema),
    /// A reference to another schema definition. Used to avoid infinite recursion.
    Reference(String),
    Number,
    String,
    Boolean
}

/// A schema definition for an arbitrary object type (hashmap).
#[derive(Reflect, Serialize, Deserialize)]
pub struct ObjectSchema {
    pub key_schema: Box<Schema>,
    pub value_schema: Box<Schema>
}

/// A field in a schema definition.
#[derive(Reflect, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub ty: Schema,
    pub docs: Option<String>
}

/// An enum schema definition.
#[derive(Reflect, Serialize, Deserialize)]
pub struct EnumValue {
    pub variants: Vec<EnumVariant>,
    pub tag_name: String,
    pub content_subfield: Option<String>
}

/// A variant in an enum schema definition.
#[derive(Reflect, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<Schema>
}