mod float_arithmetic {
    use crate::{register_node, render::effects::node_editor::{types::FloatValue, TypeInfo}};
    use super::super::node::{PortInfo, SimpleTypedNode};

    register_node!("AddNode", SimpleTypedNode::new(
        "Add",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0 + b.0),)),
    ));

    register_node!("SubtractNode", SimpleTypedNode::new(
        "Subtract",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0 - b.0),)),
    ));

    register_node!("MultiplyNode", SimpleTypedNode::new(
        "Multiply",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0 * b.0),)),
    ));

    register_node!("DivideNode", SimpleTypedNode::new(
        "Divide",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0 / b.0),)),
    ));

    register_node!("ClampNode", SimpleTypedNode::new(
        "Clamp",
        vec![
            PortInfo {
                name: "value".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "min".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "max".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value, min, max): (FloatValue, FloatValue, FloatValue)| {
            Ok((FloatValue(value.0.clamp(min.0, max.0)),))
        },
    ));

    register_node!("SinNode", SimpleTypedNode::new(
        "Sin",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Float,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.sin()),)),
    ));

    register_node!("CosNode", SimpleTypedNode::new(
        "Cos",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Float,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.cos()),)),
    ));

    register_node!("TanNode", SimpleTypedNode::new(
        "Tan",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Float,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.tan()),)),
    ));

    register_node!("SqrtNode", SimpleTypedNode::new(
        "Sqrt",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Float,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.sqrt()),)),
    ));

    register_node!("PowNode", SimpleTypedNode::new(
        "Pow",
        vec![
            PortInfo {
                name: "base".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "exponent".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(base, exponent): (FloatValue, FloatValue)| Ok((FloatValue(base.0.powf(exponent.0)),)),
    ));

    register_node!("AbsNode", SimpleTypedNode::new(
        "Abs",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Float,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.abs()),)),
    ));

    register_node!("MaxNode", SimpleTypedNode::new(
        "Max",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0.max(b.0)),)),
    ));

    register_node!("MinNode", SimpleTypedNode::new(
        "Min",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((FloatValue(a.0.min(b.0)),)),
    ));

    register_node!("FloorNode", SimpleTypedNode::new(
        "Floor",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Float,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.floor()),)),
    ));

    register_node!("CeilNode", SimpleTypedNode::new(
        "Ceil",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Float,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.ceil()),)),
    ));

    register_node!("RoundNode", SimpleTypedNode::new(
        "Round",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Float,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Float,
        }],
        |(value,): (FloatValue,)| Ok((FloatValue(value.0.round()),)),
    ));
}

mod comparisons {
    use crate::{register_node, render::effects::node_editor::{types::{FloatValue, BoolValue}, TypeInfo}};
    use super::super::node::{PortInfo, SimpleTypedNode};

    register_node!("EqualNode", SimpleTypedNode::new(
        "Equal",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 == b.0),)),
    ));

    register_node!("NotEqualNode", SimpleTypedNode::new(
        "NotEqual",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 != b.0),)),
    ));

    register_node!("LessThanNode", SimpleTypedNode::new(
        "LessThan",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 < b.0),)),
    ));

    register_node!("LessThanOrEqualNode", SimpleTypedNode::new(
        "LessThanOrEqual",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 <= b.0),)),
    ));

    register_node!("GreaterThanNode", SimpleTypedNode::new(
        "GreaterThan",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 > b.0),)),
    ));

    register_node!("GreaterThanOrEqualNode", SimpleTypedNode::new(
        "GreaterThanOrEqual",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(a, b): (FloatValue, FloatValue)| Ok((BoolValue(a.0 >= b.0),)),
    ));

    register_node!("AndNode", SimpleTypedNode::new(
        "And",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Bool,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Bool,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(a, b): (BoolValue, BoolValue)| Ok((BoolValue(a.0 && b.0),)),
    ));

    register_node!("OrNode", SimpleTypedNode::new(
        "Or",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Bool,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Bool,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(a, b): (BoolValue, BoolValue)| Ok((BoolValue(a.0 || b.0),)),
    ));

    register_node!("NotNode", SimpleTypedNode::new(
        "Not",
        vec![PortInfo {
            name: "value".into(),
            type_info: TypeInfo::Bool,
        }],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Bool,
        }],
        |(value,): (BoolValue,)| Ok((BoolValue(!value.0),)),
    ));
}

mod color {
    use crate::{register_node, render::{effects::node_editor::{types::{ColorValue, FloatValue}, TypeInfo}, frame::PixelColor}};
    use super::super::node::{PortInfo, SimpleTypedNode};

    // RGB manipulation

    register_node!("CombineRGBNode", SimpleTypedNode::new(
        "CombineRGB",
        vec![
            PortInfo {
                name: "r".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "g".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Color,
        }],
        |(r, g, b): (FloatValue, FloatValue, FloatValue)| Ok((ColorValue(
            PixelColor::new((r.0 * 255.) as u8, (g.0 * 255.) as u8, (b.0 * 255.) as u8, 1.)
        ),)),
    ));

    register_node!("SplitRGBNode", SimpleTypedNode::new(
        "SplitRGB",
        vec![PortInfo {
            name: "color".into(),
            type_info: TypeInfo::Color,
        }],
        vec![
            PortInfo {
                name: "r".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "g".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
        ],
        |(color,): (ColorValue,)| Ok((
            FloatValue(color.0.r as f64 / 255.),
            FloatValue(color.0.g as f64 / 255.),
            FloatValue(color.0.b as f64 / 255.),
        )),
    ));

    register_node!("CombineRGBANode", SimpleTypedNode::new(
        "CombineRGBA",
        vec![
            PortInfo {
                name: "r".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "g".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Color,
        }],
        |(r, g, b, a): (FloatValue, FloatValue, FloatValue, FloatValue)| Ok((ColorValue(
            PixelColor::new((r.0 * 255.) as u8, (g.0 * 255.) as u8, (b.0 * 255.) as u8, a.0)
        ),)),
    ));

    register_node!("SplitRGBANode", SimpleTypedNode::new(
        "SplitRGBA",
        vec![PortInfo {
            name: "color".into(),
            type_info: TypeInfo::Color,
        }],
        vec![
            PortInfo {
                name: "r".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "g".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Float,
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
    register_node!("CombineHSLNode", SimpleTypedNode::new(
        "CombineHSL",
        vec![
            PortInfo {
                name: "hue".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "saturation".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "lightness".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Color,
        }],
        |(hue, saturation, lightness): (FloatValue, FloatValue, FloatValue)| {
            let color = PixelColor::from_hsl(hue.0, saturation.0, lightness.0, 1.);
            Ok((ColorValue(color),))
        },
    ));

    register_node!("SplitHSLNode", SimpleTypedNode::new(
        "SplitHSL",
        vec![PortInfo {
            name: "color".into(),
            type_info: TypeInfo::Color,
        }],
        vec![
            PortInfo {
                name: "hue".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "saturation".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "lightness".into(),
                type_info: TypeInfo::Float,
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

    register_node!("CombineHSLANode", SimpleTypedNode::new(
        "CombineHSLA",
        vec![
            PortInfo {
                name: "hue".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "saturation".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "lightness".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "alpha".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Color,
        }],
        |(hue, saturation, lightness, alpha): (FloatValue, FloatValue, FloatValue, FloatValue)| {
            let color = PixelColor::from_hsl(hue.0, saturation.0, lightness.0, alpha.0);
            Ok((ColorValue(color),))
        },
    ));

    register_node!("SplitHSLANode", SimpleTypedNode::new(
        "SplitHSLA",
        vec![PortInfo {
            name: "color".into(),
            type_info: TypeInfo::Color,
        }],
        vec![
            PortInfo {
                name: "hue".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "saturation".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "lightness".into(),
                type_info: TypeInfo::Float,
            },
            PortInfo {
                name: "alpha".into(),
                type_info: TypeInfo::Float,
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
    register_node!("LerpColorNode", SimpleTypedNode::new(
        "LerpColor",
        vec![
            PortInfo {
                name: "a".into(),
                type_info: TypeInfo::Color,
            },
            PortInfo {
                name: "b".into(),
                type_info: TypeInfo::Color,
            },
            PortInfo {
                name: "t".into(),
                type_info: TypeInfo::Float,
            },
        ],
        vec![PortInfo {
            name: "result".into(),
            type_info: TypeInfo::Color,
        }],
        |(a, b, t): (ColorValue, ColorValue, FloatValue)| Ok((ColorValue(
            a.0.lerp(&b.0, t.0),
        ),)),
    ));
}