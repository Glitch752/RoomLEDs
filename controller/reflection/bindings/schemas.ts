import type { Schema } from './schema_types';

// Define the schema registry. To avoid infinite recursion, we use a object and reference
// schemas by name.
export let schemas: { [key: string]: Schema } = { /* Items set later in this file */ };

/**
 * Generated schema.
 * Tagged with "type".
 * A schema definition for a type.
 */
export const SchemaSchema: Schema = {
  "type": "Enum",
  "content": [
    {
      "name": "Struct",
      "value": {
        "type": "ArrayOf",
        "content": {
          "type": "Reference",
          "content": "SchemaField"
        }
      }
    },
    {
      "name": "Enum",
      "value": {
        "type": "ArrayOf",
        "content": {
          "type": "Reference",
          "content": "EnumVariant"
        }
      }
    },
    {
      "name": "Optional",
      "value": {
        "type": "Reference",
        "content": "Schema"
      }
    },
    {
      "name": "ArrayOf",
      "value": {
        "type": "Reference",
        "content": "Schema"
      }
    },
    {
      "name": "TupleOf",
      "value": {
        "type": "ArrayOf",
        "content": {
          "type": "Reference",
          "content": "Schema"
        }
      }
    },
    {
      "name": "Reference",
      "value": {
        "type": "String"
      }
    },
    {
      "name": "Number",
      "value": null
    },
    {
      "name": "String",
      "value": null
    },
    {
      "name": "Boolean",
      "value": null
    }
  ]
};
schemas["Schema"] = SchemaSchema;

/**
 * Generated schema.
 * A field in a schema definition.
 */
export const SchemaFieldSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "name",
      "ty": {
        "type": "String"
      }
    },
    {
      "name": "ty",
      "ty": {
        "type": "Reference",
        "content": "Schema"
      }
    }
  ]
};
schemas["SchemaField"] = SchemaFieldSchema;

/**
 * Generated schema.
 * A variant in an enum schema definition.
 */
export const EnumVariantSchema: Schema = {
  "type": "Struct",
  "content": [
    {
      "name": "name",
      "ty": {
        "type": "String"
      }
    },
    {
      "name": "value",
      "ty": {
        "type": "Optional",
        "content": {
          "type": "Reference",
          "content": "Schema"
        }
      }
    }
  ]
};
schemas["EnumVariant"] = EnumVariantSchema;

