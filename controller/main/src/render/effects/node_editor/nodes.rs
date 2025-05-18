mod float_arithmetic {
    use crate::{register_node, render::effects::node_editor::{types::FloatValue, TypeInfo}};
    use super::super::{PortInfo, TypedNode};

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

    register_node!("SubtractNode", TypedNode::new(
        "Subtract",
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
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0 - b.0),)),
    ));

    register_node!("MultiplyNode", TypedNode::new(
        "Multiply",
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
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0 * b.0),)),
    ));

    register_node!("DivideNode", TypedNode::new(
        "Divide",
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
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0 / b.0),)),
    ));

    register_node!("ClampNode", TypedNode::new(
        "Clamp",
        vec![
            PortInfo {
                name: "value".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "min".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "max".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value, min, max): (FloatValue, FloatValue, FloatValue)| {
            Ok((FloatValue(value.0.clamp(min.0, max.0)),))
        },
    ));

    register_node!("SinNode", TypedNode::new(
        "Sin",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::FLOAT,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.sin()),)),
    ));

    register_node!("CosNode", TypedNode::new(
        "Cos",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::FLOAT,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.cos()),)),
    ));

    register_node!("TanNode", TypedNode::new(
        "Tan",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::FLOAT,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.tan()),)),
    ));

    register_node!("SqrtNode", TypedNode::new(
        "Sqrt",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::FLOAT,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.sqrt()),)),
    ));

    register_node!("PowNode", TypedNode::new(
        "Pow",
        vec![
            PortInfo {
                name: "base".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "exponent".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(base, exponent): (FloatValue, FloatValue)| Ok((FloatValue(base.0.powf(exponent.0)),)),
    ));

    register_node!("AbsNode", TypedNode::new(
        "Abs",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::FLOAT,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.abs()),)),
    ));

    register_node!("MaxNode", TypedNode::new(
        "Max",
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
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0.max(b.0)),)),
    ));

    register_node!("MinNode", TypedNode::new(
        "Min",
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
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0.min(b.0)),)),
    ));

    register_node!("FloorNode", TypedNode::new(
        "Floor",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::FLOAT,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.floor()),)),
    ));

    register_node!("CeilNode", TypedNode::new(
        "Ceil",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::FLOAT,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.ceil()),)),
    ));

    register_node!("RoundNode", TypedNode::new(
        "Round",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::FLOAT,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::FLOAT,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.round()),)),
    ));
}

mod comparisons {
    use crate::{register_node, render::effects::node_editor::{types::{FloatValue, BoolValue}, TypeInfo}};
    use super::super::{PortInfo, TypedNode};

    register_node!("EqualNode", TypedNode::new(
        "Equal",
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
            type_info: TypeInfo::BOOL,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 == b.0),)),
    ));

    register_node!("NotEqualNode", TypedNode::new(
        "NotEqual",
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
            type_info: TypeInfo::BOOL,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 != b.0),)),
    ));

    register_node!("LessThanNode", TypedNode::new(
        "LessThan",
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
            type_info: TypeInfo::BOOL,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 < b.0),)),
    ));

    register_node!("LessThanOrEqualNode", TypedNode::new(
        "LessThanOrEqual",
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
            type_info: TypeInfo::BOOL,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 <= b.0),)),
    ));

    register_node!("GreaterThanNode", TypedNode::new(
        "GreaterThan",
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
            type_info: TypeInfo::BOOL,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 > b.0),)),
    ));

    register_node!("GreaterThanOrEqualNode", TypedNode::new(
        "GreaterThanOrEqual",
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
            type_info: TypeInfo::BOOL,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 >= b.0),)),
    ));

    register_node!("AndNode", TypedNode::new(
        "And",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::BOOL,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::BOOL,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::BOOL,
        }],
        |(a, b): (BoolValue, BoolValue)| Ok((BoolValue(a.0 && b.0),)),
    ));

    register_node!("OrNode", TypedNode::new(
        "Or",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::BOOL,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::BOOL,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::BOOL,
        }],
        |(a, b): (BoolValue, BoolValue)| Ok((BoolValue(a.0 || b.0),)),
    ));

    register_node!("NotNode", TypedNode::new(
        "Not",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::BOOL,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::BOOL,
        }],
        |(value,): (BoolValue,)| Ok((BoolValue(!value.0),)),
    ));
}

mod color {
    use crate::{register_node, render::{effects::node_editor::{types::{ColorValue, FloatValue}, TypeInfo}, frame::PixelColor}};
    use super::super::{PortInfo, TypedNode};

    // RGB manipulation

    register_node!("CombineRGBNode", TypedNode::new(
        "CombineRGB",
        vec![
            PortInfo {
                name: "r".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "g".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::COLOR,
        }],
        |(r, g, b): (FloatValue, FloatValue, FloatValue)| Ok((ColorValue(
            PixelColor::new((r.0 * 255.) as u8, (g.0 * 255.) as u8, (b.0 * 255.) as u8, 1.)
        ),)),
    ));

    register_node!("SplitRGBNode", TypedNode::new(
        "SplitRGB",
        vec![PortInfo {
            name: "color".into(),
            type_info: TypeInfo::COLOR,
        }],
        vec![
            PortInfo {
                name: "r".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "g".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        |(color,): (ColorValue,)| Ok((
            FloatValue(color.0.r as f64 / 255.),
            FloatValue(color.0.g as f64 / 255.),
            FloatValue(color.0.b as f64 / 255.),
        )),
    ));

    register_node!("CombineRGBANode", TypedNode::new(
        "CombineRGBA",
        vec![
            PortInfo {
                name: "r".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "g".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::COLOR,
        }],
        |(r, g, b, a): (FloatValue, FloatValue, FloatValue, FloatValue)| Ok((ColorValue(
            PixelColor::new((r.0 * 255.) as u8, (g.0 * 255.) as u8, (b.0 * 255.) as u8, a.0)
        ),)),
    ));

    register_node!("SplitRGBANode", TypedNode::new(
        "SplitRGBA",
        vec![PortInfo {
            name: "color".into(),
            type_info: TypeInfo::COLOR,
        }],
        vec![
            PortInfo {
                name: "r".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "g".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        |(color,): (ColorValue,)| Ok((
            FloatValue(color.0.r as f64 / 255.),
            FloatValue(color.0.g as f64 / 255.),
            FloatValue(color.0.b as f64 / 255.),
            FloatValue(color.0.alpha as f64),
        )),
    ));

    // HSL manipulation
    register_node!("CombineHSLNode", TypedNode::new(
        "CombineHSL",
        vec![
            PortInfo {
                name: "hue".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "saturation".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "lightness".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::COLOR,
        }],
        |(hue, saturation, lightness): (FloatValue, FloatValue, FloatValue)| {
            let color = PixelColor::from_hsl(hue.0, saturation.0, lightness.0, 1.);
            Ok((ColorValue(color),))
        },
    ));

    register_node!("SplitHSLNode", TypedNode::new(
        "SplitHSL",
        vec![PortInfo {
            name: "color".into(),
            type_info: TypeInfo::COLOR,
        }],
        vec![
            PortInfo {
                name: "hue".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "saturation".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "lightness".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        |(color,): (ColorValue,)| {
            let hsl = color.0.to_hsl();
            Ok((
                FloatValue(hsl.0),
                FloatValue(hsl.1),
                FloatValue(hsl.2),
            ))
        },
    ));

    register_node!("CombineHSLANode", TypedNode::new(
        "CombineHSLA",
        vec![
            PortInfo {
                name: "hue".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "saturation".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "lightness".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "alpha".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::COLOR,
        }],
        |(hue, saturation, lightness, alpha): (FloatValue, FloatValue, FloatValue, FloatValue)| {
            let color = PixelColor::from_hsl(hue.0, saturation.0, lightness.0, alpha.0);
            Ok((ColorValue(color),))
        },
    ));

    register_node!("SplitHSLANode", TypedNode::new(
        "SplitHSLA",
        vec![PortInfo {
            name: "color".into(),
            type_info: TypeInfo::COLOR,
        }],
        vec![
            PortInfo {
                name: "hue".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "saturation".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "lightness".into(),
                type_info: TypeInfo::FLOAT,
            },
            PortInfo {
                name: "alpha".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        |(color,): (ColorValue,)| {
            let alpha = color.0.alpha;
            let hsl = color.0.to_hsl();
            Ok((
                FloatValue(hsl.0),
                FloatValue(hsl.1),
                FloatValue(hsl.2),
                FloatValue(alpha),
            ))
        },
    ));

    // Color manipulation
    register_node!("LerpColorNode", TypedNode::new(
        "LerpColor",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::COLOR,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::COLOR,
            },
            PortInfo {
                name: "t".into(),
                type_info: TypeInfo::FLOAT,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::COLOR,
        }],
        |(a, b, t): (ColorValue, ColorValue, FloatValue)| Ok((ColorValue(
            a.0.lerp(&b.0, t.0),
        ),)),
    ));
}