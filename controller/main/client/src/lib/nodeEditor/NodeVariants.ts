export type NodeDataValueDescriptor = ({
    type: "number",
    default: number,
} | {
    type: "enum",
    options: string[],
    default: string
} | {
    type: "boolean",
    default: boolean
} | {
    type: "color",
    default: string // hex color
} | {
    type: "string",
    default: string
}) & {
    label: string;
};
export type NodeDataValue = number | string | boolean;

export type NodeVariantDescriptor = {
    name: string;
    description: string;

    inputs: {
        label: string;
        type: NodeDataType;
    }[];
    outputs: {
        label: string;
        type: NodeDataType;
    }[];

    zoneBehavior: {
        type: 'start',
        endVariant: NodeVariant
    } | {
        type: 'end',
        startVariant: NodeVariant
    } | null;

    dataValues: {
        [key: string]: NodeDataValueDescriptor
    }
}

export enum NodeVariant {
    // Inputs
    InputNumber,
    InputBoolean,
    InputColor,
    InputFrame,
    InputVector2,
    Time,

    // Outputs
    OutputNumber,
    OutputBoolean,
    OutputColor,
    OutputFrame,
    OutputVector2,
    
    // Constants
    NumberConstant,
    BooleanConstant,
    ColorConstant,
    FrameConstant,
    Vector2Constant,

    // Combine/split
    MakeColorRGB,
    SplitColorRGB,
    MakeColorHSL,
    SplitColorHSL,

    MakeVector2,
    SplitVector2,
    BlankFrame,

    // Zones
    ForEachPixelStart,
    ForEachPixelEnd,
    FrameRepeatStart,
    FrameRepeatEnd,

    // Frame pixel operations
    SetPixelColor,
    GetPixelColor,

    // Math
    UnaryNumberOperation,
    BinaryNumberOperation,
    ComparisonOperation,
    UnaryLogicalOperation,
    BinaryLogicalOperation,
    BinaryVector2Operation,
    // TODO: scalar vector2 operations and stuff
    
    // Blending
    ColorBlend,
    FrameBlend,
    
    // Conditional
    ConditionalSelect

    // TODO: List operations
};

export enum NodeDataType {
    Number,
    Color,
    Boolean,
    Frame,
    Vector2,

    // i am NOT doing generic types for ts
    NumberList,
    ColorList,
    BooleanList,
    FrameList,
    Vector2List
}

export type NodeDataTypeDescriptor = {
    label: string;
    primaryColor: string;
    lightColor: string;
    alphaBackgroundColor: string;
}

export const nodeDataTypeInfo: {
    [type in NodeDataType]: NodeDataTypeDescriptor
} = {
    [NodeDataType.Boolean]: {
        primaryColor: "#ff6666",
        alphaBackgroundColor: "#ff000055",
        lightColor: "#ff9999",
        label: "Boolean"
    },
    [NodeDataType.Number]: {
        primaryColor: "#66ccff",
        alphaBackgroundColor: "#00aaff55",
        lightColor: "#99ddff",
        label: "Number"
    },
    [NodeDataType.Color]: {
        primaryColor: "#ffcc66",
        alphaBackgroundColor: "#ffaa0055",
        lightColor: "#ffdd99",
        label: "Color"
    },
    [NodeDataType.Frame]: {
        primaryColor: "#aa66ff",
        alphaBackgroundColor: "#8800ff55",
        lightColor: "#cc99ff",
        label: "Frame"
    },
    [NodeDataType.Vector2]: {
        primaryColor: "#66ff99",
        alphaBackgroundColor: "#00ff7755",
        lightColor: "#99ffcc",
        label: "Vector2"
    },

    [NodeDataType.BooleanList]: {
        primaryColor: "#ee4444",
        alphaBackgroundColor: "#ee444455",
        lightColor: "#ff7777",
        label: "Boolean List"
    },
    [NodeDataType.NumberList]: {
        primaryColor: "#3399ee",
        alphaBackgroundColor: "#3399ee55",
        lightColor: "#66bbff",
        label: "Number List"
    },
    [NodeDataType.ColorList]: {
        primaryColor: "#eebb33",
        alphaBackgroundColor: "#eebb3355",
        lightColor: "#ffd966",
        label: "Color List"
    },
    [NodeDataType.FrameList]: {
        primaryColor: "#9933ee",
        alphaBackgroundColor: "#9933ee55",
        lightColor: "#bb66ff",
        label: "Frame List"
    },
    [NodeDataType.Vector2List]: {
        primaryColor: "#33ee88",
        alphaBackgroundColor: "#33ee8855",
        lightColor: "#66ffbb",
        label: "Vector2 List"
    }
};






const input = (name: string, type: NodeDataType): NodeVariantDescriptor => ({
    name: `${name}`,
    description: `An input node that provides ${
        name.includes("input") ? "a" : "the"
    } ${name.toLowerCase()}.`,
    inputs: [],
    outputs: [{
        label: 'Value',
        type
    }],
    zoneBehavior: null,
    dataValues: {
        name: {
            type: "string",
            default: `${name}`,
            label: "Name"
        }
    }
});

const output = (name: string, type: NodeDataType): NodeVariantDescriptor => ({
    name: `${name}`,
    description: `An output node that outputs a ${name.toLowerCase()}.`,
    inputs: [{
        label: 'Value',
        type
    }],
    outputs: [],
    zoneBehavior: null,
    dataValues: {
        name: {
            type: "string",
            default: `${name}`,
            label: "Name"
        }
    }
});

const constant = (name: string, type: NodeDataType, defaultValue: number | boolean | string): NodeVariantDescriptor => ({
    name: `${name}`,
    description: `A constant ${name.toLowerCase()} value.`,
    inputs: [],
    outputs: [{
        label: 'out',
        type
    }],
    zoneBehavior: null,
    dataValues: {
        value: {
            type: ({
                [NodeDataType.Number]: "number",
                [NodeDataType.Boolean]: "boolean",
                [NodeDataType.Color]: "color",
                [NodeDataType.Frame]: "number", // erm idk
                [NodeDataType.Vector2]: "number" // erm idk
            } as const)[type as number] ?? "number",
            default: defaultValue as any,
            label: "Value"
        }
    }
});

export const nodeVariantInfo: {
    [type in NodeVariant]: NodeVariantDescriptor
} = {
    // Inputs
    [NodeVariant.InputNumber]: input("Number input", NodeDataType.Number),
    [NodeVariant.InputBoolean]: input("Boolean input", NodeDataType.Boolean),
    [NodeVariant.InputColor]: input("Color input", NodeDataType.Color),
    [NodeVariant.InputFrame]: input("Frame input", NodeDataType.Frame),
    [NodeVariant.InputVector2]: input("Vector2 input", NodeDataType.Vector2),
    [NodeVariant.Time]: input("Time", NodeDataType.Number),

    // Outputs
    [NodeVariant.OutputNumber]: output("Number output", NodeDataType.Number),
    [NodeVariant.OutputBoolean]: output("Boolean output", NodeDataType.Boolean),
    [NodeVariant.OutputColor]: output("Color output", NodeDataType.Color),
    [NodeVariant.OutputFrame]: output("Frame output", NodeDataType.Frame),
    [NodeVariant.OutputVector2]: output("Vector2 output", NodeDataType.Vector2),

    // Constants
    [NodeVariant.NumberConstant]: constant("Number constant", NodeDataType.Number, 0),
    [NodeVariant.BooleanConstant]: constant("Boolean constant", NodeDataType.Boolean, false),
    [NodeVariant.ColorConstant]: constant("Color constant", NodeDataType.Color, "#ffffff"),
    [NodeVariant.FrameConstant]: constant("Frame constant", NodeDataType.Frame, 0),
    [NodeVariant.Vector2Constant]: constant("Vector2 constant", NodeDataType.Vector2, 0),

    // Combine/split
    [NodeVariant.MakeColorRGB]: {
        name: "Make Color (RGB)",
        description: "Combines red, green, and blue components into a color.",
        inputs: [
            { label: "R", type: NodeDataType.Number },
            { label: "G", type: NodeDataType.Number },
            { label: "B", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Color", type: NodeDataType.Color }
        ],
        zoneBehavior: null,
        dataValues: {}
    },
    [NodeVariant.SplitColorRGB]: {
        name: "Split Color (RGB)",
        description: "Splits a color into its red, green, and blue components.",
        inputs: [
            { label: "Color", type: NodeDataType.Color }
        ],
        outputs: [
            { label: "R", type: NodeDataType.Number },
            { label: "G", type: NodeDataType.Number },
            { label: "B", type: NodeDataType.Number }
        ],
        zoneBehavior: null,
        dataValues: {}
    },
    [NodeVariant.MakeColorHSL]: {
        name: "Make Color (HSL)",
        description: "Combines hue, saturation, and lightness components into a color.",
        inputs: [
            { label: "H", type: NodeDataType.Number },
            { label: "S", type: NodeDataType.Number },
            { label: "L", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Color", type: NodeDataType.Color }
        ],
        zoneBehavior: null,
        dataValues: {}
    },
    [NodeVariant.SplitColorHSL]: {
        name: "Split Color (HSL)",
        description: "Splits a color into its hue, saturation, and lightness components.",
        inputs: [
            { label: "Color", type: NodeDataType.Color }
        ],
        outputs: [
            { label: "H", type: NodeDataType.Number },
            { label: "S", type: NodeDataType.Number },
            { label: "L", type: NodeDataType.Number }
        ],
        zoneBehavior: null,
        dataValues: {}
    },

    [NodeVariant.MakeVector2]: {
        name: "Make Vector2",
        description: "Combines X and Y components into a Vector2.",
        inputs: [
            { label: "X", type: NodeDataType.Number },
            { label: "Y", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Vector2", type: NodeDataType.Vector2 }
        ],
        zoneBehavior: null,
        dataValues: {}
    },
    [NodeVariant.SplitVector2]: {
        name: "Split Vector2",
        description: "Splits a Vector2 into its X and Y components.",
        inputs: [
            { label: "Vector2", type: NodeDataType.Vector2 }
        ],
        outputs: [
            { label: "X", type: NodeDataType.Number },
            { label: "Y", type: NodeDataType.Number }
        ],
        zoneBehavior: null,
        dataValues: {}
    },
    [NodeVariant.BlankFrame]: {
        name: "Blank Frame",
        description: "Creates a blank frame of the correct size with no pixels set.",
        inputs: [],
        outputs: [
            { label: "Frame", type: NodeDataType.Frame }
        ],
        zoneBehavior: null,
        dataValues: {}
    },

    // Zones
    [NodeVariant.ForEachPixelStart]: {
        name: "For Each Pixel (Start)",
        description: "Starts a zone that processes each pixel in the input frame individually.",
        inputs: [
            { label: "Frame", type: NodeDataType.Frame }
        ],
        outputs: [
            { label: "Pixel Index", type: NodeDataType.Number },
            { label: "Pixel Color", type: NodeDataType.Color }
        ],
        zoneBehavior: {
            type: 'start',
            endVariant: NodeVariant.ForEachPixelEnd
        },
        dataValues: {}
    },
    [NodeVariant.ForEachPixelEnd]: {
        name: "For Each Pixel (End)",
        description: "Ends a For Each Pixel zone, outputting the modified frame.",
        inputs: [
            { label: "Pixel Color", type: NodeDataType.Color }
        ],
        outputs: [
            { label: "Frame", type: NodeDataType.Frame }
        ],
        zoneBehavior: {
            type: 'end',
            startVariant: NodeVariant.ForEachPixelStart
        },
        dataValues: {}
    },

    [NodeVariant.FrameRepeatStart]: {
        name: "Frame Repeat (Start)",
        description: "Starts a zone that repeats an operation on a frame multiple times.",
        inputs: [
            { label: "Frame", type: NodeDataType.Frame },
            { label: "Repeat Count", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Iteration Index", type: NodeDataType.Number },
            { label: "Frame", type: NodeDataType.Frame }
        ],
        zoneBehavior: {
            type: 'start',
            endVariant: NodeVariant.FrameRepeatEnd
        },
        dataValues: {}
    },
    [NodeVariant.FrameRepeatEnd]: {
        name: "Frame Repeat (End)",
        description: "Ends a Frame Repeat zone, outputting the final frame.",
        inputs: [
            { label: "Frame", type: NodeDataType.Frame }
        ],
        outputs: [
            { label: "Frame", type: NodeDataType.Frame }
        ],
        zoneBehavior: {
            type: 'end',
            startVariant: NodeVariant.FrameRepeatStart
        },
        dataValues: {}
    },

    // Frame pixel operations
    [NodeVariant.SetPixelColor]: {
        name: "Set Pixel Color",
        description: "Sets the color of a specific pixel in a frame.",
        inputs: [
            { label: "Frame", type: NodeDataType.Frame },
            { label: "Pixel Index", type: NodeDataType.Number },
            { label: "Color", type: NodeDataType.Color }
        ],
        outputs: [
            { label: "Frame", type: NodeDataType.Frame }
        ],
        zoneBehavior: null,
        dataValues: {}
    },
    [NodeVariant.GetPixelColor]: {
        name: "Get Pixel Color",
        description: "Gets the color of a specific pixel in a frame.",
        inputs: [
            { label: "Frame", type: NodeDataType.Frame },
            { label: "Pixel Index", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Color", type: NodeDataType.Color }
        ],
        zoneBehavior: null,
        dataValues: {}
    },
    
    // Math
    [NodeVariant.UnaryNumberOperation]: {
        name: "Unary Number Operation",
        description: "Performs a unary operation on a number (e.g., negation, absolute value).",
        inputs: [
            { label: "Input", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Output", type: NodeDataType.Number }
        ],
        zoneBehavior: null,
        dataValues: {
            operation: {
                type: "enum",
                options: ["negate", "absolute", "sqrt", "sin", "cos", "tan"],
                default: "negate",
                label: "Operation"
            }
        }
    },
    [NodeVariant.BinaryNumberOperation]: {
        name: "Binary Number Operation",
        description: "Performs a binary operation on two numbers (e.g., addition, subtraction).",
        inputs: [
            { label: "A", type: NodeDataType.Number },
            { label: "B", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Result", type: NodeDataType.Number }
        ],
        zoneBehavior: null,
        dataValues: {
            operation: {
                type: "enum",
                options: ["add", "subtract", "multiply", "divide", "power", "modulo", "min", "max"],
                default: "add",
                label: "Operation"
            }
        }
    },
    [NodeVariant.ComparisonOperation]: {
        name: "Comparison Operation",
        description: "Compares two numbers and outputs a boolean result.",
        inputs: [
            { label: "A", type: NodeDataType.Number },
            { label: "B", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Result", type: NodeDataType.Boolean }
        ],
        zoneBehavior: null,
        dataValues: {
            operation: {
                type: "enum",
                options: ["equal", "not equal", "less than", "less than or equal", "greater than", "greater than or equal"],
                default: "equal",
                label: "Operation"
            }
        }
    },
    [NodeVariant.UnaryLogicalOperation]: {
        name: "Unary Logical Operation",
        description: "Performs a unary operation on a boolean value.",
        inputs: [
            { label: "Input", type: NodeDataType.Boolean }
        ],
        outputs: [
            { label: "Output", type: NodeDataType.Boolean }
        ],
        zoneBehavior: null,
        dataValues: {
            operation: {
                type: "enum",
                options: ["NOT"],
                default: "NOT",
                label: "Operation"
            }
        }
    },
    [NodeVariant.BinaryLogicalOperation]: {
        name: "Binary Logical Operation",
        description: "Performs a logical operation on two boolean values.",
        inputs: [
            { label: "A", type: NodeDataType.Boolean },
            { label: "B", type: NodeDataType.Boolean }
        ],
        outputs: [
            { label: "Result", type: NodeDataType.Boolean }
        ],
        zoneBehavior: null,
        dataValues: {
            operation: {
                type: "enum",
                options: ["AND", "OR", "XOR", "NAND", "NOR"],
                default: "AND",
                label: "Operation"
            }
        }
    },
    [NodeVariant.BinaryVector2Operation]: {
        name: "Binary Vector2 Operation",
        description: "Performs an operation on two Vector2 values.",
        inputs: [
            { label: "A", type: NodeDataType.Vector2 },
            { label: "B", type: NodeDataType.Vector2 }
        ],
        outputs: [
            { label: "Result", type: NodeDataType.Vector2 }
        ],
        zoneBehavior: null,
        dataValues: {
            operation: {
                type: "enum",
                options: ["add", "subtract", "multiply", "divide"],
                default: "add",
                label: "Operation"
            }
        }
    },

    // Blending
    [NodeVariant.ColorBlend]: {
        name: "Color Blend",
        description: "Blends two colors together based on a blend mode and factor.",
        inputs: [
            { label: "Color A", type: NodeDataType.Color },
            { label: "Color B", type: NodeDataType.Color },
            { label: "Factor", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Result", type: NodeDataType.Color }
        ],
        zoneBehavior: null,
        dataValues: {
            blendMode: {
                type: "enum",
                options: ["normal", "multiply", "screen", "overlay"],
                default: "normal",
                label: "Operation"
            }
        }
    },
    [NodeVariant.FrameBlend]: {
        name: "Frame Blend",
        description: "Blends two frames together based on a blend mode and factor.",
        inputs: [
            { label: "Frame A", type: NodeDataType.Frame },
            { label: "Frame B", type: NodeDataType.Frame },
            { label: "Factor", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Result", type: NodeDataType.Frame }
        ],
        zoneBehavior: null,
        dataValues: {
            blendMode: {
                type: "enum",
                options: ["normal", "additive", "subtractive"],
                default: "normal",
                label: "Operation"
            }
        }
    },
    
    // Conditional
    [NodeVariant.ConditionalSelect]: {
        name: "Conditional Select",
        description: "Selects between two inputs based on a boolean condition.",
        inputs: [
            { label: "Condition", type: NodeDataType.Boolean },
            { label: "If True", type: NodeDataType.Number },
            { label: "If False", type: NodeDataType.Number }
        ],
        outputs: [
            { label: "Result", type: NodeDataType.Number }
        ],
        zoneBehavior: null,
        dataValues: {}
    },
}