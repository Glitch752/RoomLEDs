use crate::{register_node, render::effects::node_editor::{types::FloatValue, TypeInfo}};

use super::{PortInfo, TypedNode};

register_node!("AddNode", TypedNode::new(
    "Add",
    vec![
        PortInfo {
            name: "a".into(),
            type_info: TypeInfo::FLOAT,
        },
        PortInfo {
            name: "b".into(),
            type_info: TypeInfo::FLOAT,
        },
    ],
    vec![PortInfo {
        name: "result".into(),
        type_info: TypeInfo::FLOAT,
    }],
    |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0 + b.0),)),
));