import type { Schema } from './schema_types';

// Define the schema registry. To avoid infinite recursion, we use a object and reference
// schemas by name.
export let schemas: { [key: string]: Schema } = { /* Items set later in this file */ };

/**
 * Generated schema.
 * Tagged with "type".
 * A wrapper for any effect that can be rendered.
 * Used for serialization and deserialization.
 */
export const AnyEffectSchema: Schema = {
  "type": "Enum",
  "content": {
    "variants": [
      {
        "name": "AdditiveCompositor",
        "value": {
          "type": "Reference",
          "content": "AdditiveCompositorEffect"
        }
      },
      {
        "name": "AlphaCompositor",
        "value": {
          "type": "Reference",
          "content": "AlphaCompositorEffect"
        }
      },
      {
        "name": "Stripe",
        "value": {
          "type": "Reference",
          "content": "StripeEffect"
        }
      },
      {
        "name": "MusicVisualizer",
        "value": {
          "type": "Reference",
          "content": "MusicVisualizerEffect"
        }
      },
      {
        "name": "Rotate",
        "value": {
          "type": "Reference",
          "content": "RotateEffect"
        }
      },
      {
        "name": "FlashingColor",
        "value": {
          "type": "Reference",
          "content": "FlashingColorEffect"
        }
      },
      {
        "name": "SolidColor",
        "value": {
          "type": "Reference",
          "content": "SolidColorEffect"
        }
      },
      {
        "name": "WebsocketInput",
        "value": {
          "type": "Reference",
          "content": "WebsocketInputEffect"
        }
      }
    ],
    "tag_name": "type",
    "content_subfield": null
  }
};
schemas["AnyEffect"] = AnyEffectSchema;

/**
 * Generated schema.
 * An additive compositor composites other effects together using additive blending.
 */
export const AdditiveCompositorEffectSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "effects",
      "ty": {
        "type": "ArrayOf",
        "content": {
          "type": "Reference",
          "content": "AnyEffect"
        }
      },
      "docs": "The effects to be composited together. The output of every effect will be added together."
    }
  ]
};
schemas["AdditiveCompositorEffect"] = AdditiveCompositorEffectSchema;

/**
 * Generated schema.
 * Tagged with "type".
 * A wrapper for any temporary effect that can be rendered.
 * Used for serialization and deserialization.
 */
export const AnyTemporaryEffectSchema: Schema = {
  "type": "Enum",
  "content": {
    "variants": [
      {
        "name": "TemporaryEffectWrapper",
        "value": {
          "type": "Reference",
          "content": "DurationTemporaryEffect"
        }
      }
    ],
    "tag_name": "type",
    "content_subfield": null
  }
};
schemas["AnyTemporaryEffect"] = AnyTemporaryEffectSchema;

/**
 * Generated schema.
 */
export const DurationTemporaryEffectSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "duration",
      "ty": {
        "type": "Number"
      },
      "docs": null
    },
    {
      "name": "effect",
      "ty": {
        "type": "Reference",
        "content": "AnyEffect"
      },
      "docs": null
    }
  ]
};
schemas["DurationTemporaryEffect"] = DurationTemporaryEffectSchema;

/**
 * Generated schema.
 * An alpha compositor composites other effects together using alpha blending.
 */
export const AlphaCompositorEffectSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "effects",
      "ty": {
        "type": "ArrayOf",
        "content": {
          "type": "Reference",
          "content": "AnyEffect"
        }
      },
      "docs": "The effects to be composited together. The output of every effect will be composited based on their order and transparency."
    }
  ]
};
schemas["AlphaCompositorEffect"] = AlphaCompositorEffectSchema;

/**
 * Generated schema.
 * The stripes effect renders a rotating pattern with stripes of color.
 */
export const StripeEffectSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "stripe_width",
      "ty": {
        "type": "Number"
      },
      "docs": "The width of each stripe in pixels."
    },
    {
      "name": "stripe_colors",
      "ty": {
        "type": "ArrayOf",
        "content": {
          "type": "Reference",
          "content": "PixelColor"
        }
      },
      "docs": "The colors of the stripes."
    },
    {
      "name": "speed",
      "ty": {
        "type": "Number"
      },
      "docs": "The speed of the rotation in pixels per second."
    }
  ]
};
schemas["StripeEffect"] = StripeEffectSchema;

/**
 * Generated schema.
 * A pixel is a single unit of color data with an alpha value.
 */
export const PixelColorSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "r",
      "ty": {
        "type": "Number"
      },
      "docs": null
    },
    {
      "name": "g",
      "ty": {
        "type": "Number"
      },
      "docs": null
    },
    {
      "name": "b",
      "ty": {
        "type": "Number"
      },
      "docs": null
    },
    {
      "name": "alpha",
      "ty": {
        "type": "Number"
      },
      "docs": null
    }
  ]
};
schemas["PixelColor"] = PixelColorSchema;

/**
 * Generated schema.
 * The music visualizer effect runs a TCP socket server that listens for
 * audio data from the music visualizer client. Then, it renders the audio
 * data as a visualizer.
 */
export const MusicVisualizerEffectSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "port",
      "ty": {
        "type": "Number"
      },
      "docs": "The UDP listener that listens for audio data from the music visualizer client."
    }
  ]
};
schemas["MusicVisualizerEffect"] = MusicVisualizerEffectSchema;

/**
 * Generated schema.
 */
export const RotateEffectSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "effect",
      "ty": {
        "type": "Reference",
        "content": "AnyEffect"
      },
      "docs": "The effect to rotate"
    },
    {
      "name": "rotation",
      "ty": {
        "type": "Number"
      },
      "docs": "The number of pixels to rotate the frame by"
    }
  ]
};
schemas["RotateEffect"] = RotateEffectSchema;

/**
 * Generated schema.
 */
export const FlashingColorEffectSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "offset",
      "ty": {
        "type": "Number"
      },
      "docs": "The offset of the flashing effect, in seconds."
    },
    {
      "name": "speed",
      "ty": {
        "type": "Number"
      },
      "docs": "The speed of the flashing effect, in Hz."
    },
    {
      "name": "color_a",
      "ty": {
        "type": "Reference",
        "content": "PixelColor"
      },
      "docs": "The color of the first flashing color."
    },
    {
      "name": "color_b",
      "ty": {
        "type": "Reference",
        "content": "PixelColor"
      },
      "docs": "The color of the second flashing color."
    }
  ]
};
schemas["FlashingColorEffect"] = FlashingColorEffectSchema;

/**
 * Generated schema.
 */
export const SolidColorEffectSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "color",
      "ty": {
        "type": "Reference",
        "content": "PixelColor"
      },
      "docs": "The color of the effect."
    },
    {
      "name": "start",
      "ty": {
        "type": "Number"
      },
      "docs": "The start pixel index of the effect."
    },
    {
      "name": "stop",
      "ty": {
        "type": "Number"
      },
      "docs": "The stop pixel index of the effect."
    }
  ]
};
schemas["SolidColorEffect"] = SolidColorEffectSchema;

/**
 * Generated schema.
 */
export const WebsocketInputEffectSchema: Schema = {
  "type": "Struct",
  "content": []
};
schemas["WebsocketInputEffect"] = WebsocketInputEffectSchema;

