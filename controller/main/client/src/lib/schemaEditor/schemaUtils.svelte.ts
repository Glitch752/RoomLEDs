import type { Schema } from "@bindings/schema_types";
import { schemas } from "@bindings/schemas";

export function snakeCaseToReadable(str: string): string {
    return str
        .replace(/_/g, ' ') // Replace underscores with spaces
        .replace(/\b\w/g, char => char.toUpperCase()); // Capitalize the first letter of each word
}
export function camelCaseToReadable(str: string): string {
    return str
        .replace(/([a-z])([A-Z])/g, '$1 $2') // Add space before uppercase letters
        .replace(/\b\w/g, char => char.toUpperCase()); // Capitalize the first letter of each word
}

export function createDefaultValue(schema: Schema): any {
    switch(schema.type) {
        case "Boolean":
            return false;
        case "String":
            return "";
        case "Number":
            return 0;
        case "ArrayOf":
            return [];
        case "Enum":
            const variant = schema.content.variants[0];
            if(variant.value != null) {
                const contentValue = createDefaultValue(variant.value);
                if(schema.content.content_subfield != null) {
                    return {
                        [schema.content.tag_name]: variant.name,
                        [schema.content.content_subfield]: contentValue
                    };
                } else {
                    let v = { [schema.content.tag_name]: variant.name };
                    for(const key in contentValue) {
                        v[key] = contentValue[key];
                    }
                    return v;
                }
            } else {
                return { [schema.content.tag_name]: variant.name };
            }
        case "Struct":
            const obj: Record<string, unknown> = {};
            for(const field of schema.content) {
                obj[field.name] = createDefaultValue(field.ty);
            }
            return obj;
        case "Optional":
            return null;
        case "TupleOf":
            return schema.content.map(createDefaultValue);
        case "Reference":
            // Special cases for certain reference types we want editors for
            if (schema.content === "PixelColor") {
                // Default to white color
                return { r: 255, g: 255, b: 255, alpha: 1 };
            }

            return createDefaultValue(schemas[schema.content]);
    }
}