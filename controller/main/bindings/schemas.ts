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
  "content": [
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
  ]
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
        "type": "Reference",
        "content": "AnyEffect"
      }
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
  "content": [
    {
      "name": "TemporaryEffectWrapper",
      "value": {
        "type": "Reference",
        "content": "DurationTemporaryEffect"
      }
    }
  ]
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
        "type": "Reference",
        "content": "f64"
      }
    },
    {
      "name": "effect",
      "ty": {
        "type": "Reference",
        "content": "AnyEffect"
      }
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
        "type": "Reference",
        "content": "AnyEffect"
      }
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
        "type": "Reference",
        "content": "f64"
      }
    },
    {
      "name": "stripe_colors",
      "ty": {
        "type": "Reference",
        "content": "Vec<(u8, u8, u8)"
      }
    },
    {
      "name": "speed",
      "ty": {
        "type": "Reference",
        "content": "f64"
      }
    }
  ]
};
schemas["StripeEffect"] = StripeEffectSchema;

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
        "type": "Reference",
        "content": "u16"
      }
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
      }
    },
    {
      "name": "rotation",
      "ty": {
        "type": "Reference",
        "content": "i32"
      }
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
      "name": "time",
      "ty": {
        "type": "Reference",
        "content": "f64"
      }
    },
    {
      "name": "speed",
      "ty": {
        "type": "Reference",
        "content": "f64"
      }
    },
    {
      "name": "color",
      "ty": {
        "type": "Reference",
        "content": "Pixel"
      }
    }
  ]
};
schemas["FlashingColorEffect"] = FlashingColorEffectSchema;

/**
 * Generated schema.
 * A pixel is a single unit of color data with an alpha value.
 */
export const PixelSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "r",
      "ty": {
        "type": "Reference",
        "content": "u8"
      }
    },
    {
      "name": "g",
      "ty": {
        "type": "Reference",
        "content": "u8"
      }
    },
    {
      "name": "b",
      "ty": {
        "type": "Reference",
        "content": "u8"
      }
    },
    {
      "name": "alpha",
      "ty": {
        "type": "Reference",
        "content": "f64"
      }
    }
  ]
};
schemas["Pixel"] = PixelSchema;

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
        "content": "Pixel"
      }
    },
    {
      "name": "start",
      "ty": {
        "type": "Reference",
        "content": "u32"
      }
    },
    {
      "name": "stop",
      "ty": {
        "type": "Reference",
        "content": "u32"
      }
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

