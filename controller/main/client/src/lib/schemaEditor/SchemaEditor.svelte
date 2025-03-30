<script lang="ts" generics="T">
import type { Schema } from "@bindings/schema_types";
import type { PixelColor } from "@bindings/index";
import { schemas } from "@bindings/schemas";
import SchemaEditor from "./SchemaEditor.svelte";
import ColorPicker, { ChromeVariant, type RgbaColor } from 'svelte-awesome-color-picker';

// Yuck... what is this runes syntax (I'm just biased)
let {
    schema,
    value = $bindable(),
    name = "",
    noShell = false
}: {
    schema: Schema,
    value: T,
    name?: string,
    noShell?: boolean
} = $props();

// This eventually needs to be more generic, but it's fine for now
type EnumValue = { name: string, value?: any };

function createDefaultValue(schema: Schema): any {
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
            const variant = schema.content[0];
            if(variant.value != null) {
                return {
                    name: variant.name,
                    value: createDefaultValue(variant.value)
                };
            } else {
                return { name: variant.name };
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

// Helper function to convert a struct color to hex
function structColorToRgba(value: PixelColor): RgbaColor {
    return {
        r: value.r,
        g: value.g,
        b: value.b,
        a: value.alpha
    };
}
function rgbaToStructColor(rgba: RgbaColor | null): PixelColor {
    if (rgba === null) {
        return { r: 0, g: 0, b: 0, alpha: 1 }; // Default to black with full alpha
    }
    return {
        r: Math.round(rgba.r),
        g: Math.round(rgba.g),
        b: Math.round(rgba.b),
        alpha: rgba.a
    };
}

function snakeCaseToReadable(str: string): string {
    return str
        .replace(/_/g, ' ') // Replace underscores with spaces
        .replace(/\b\w/g, char => char.toUpperCase()); // Capitalize the first letter of each word
}
function camelCaseToReadable(str: string): string {
    return str
        .replace(/([a-z])([A-Z])/g, '$1 $2') // Add space before uppercase letters
        .replace(/\b\w/g, char => char.toUpperCase()); // Capitalize the first letter of each word
}
</script>

<div class:editor={!noShell}>
    {#if !noShell && name !== ""}
        <span class="entryName">{snakeCaseToReadable(name)}</span>
    {/if}
    {#if schema.type == "Boolean"}
        <input type="checkbox" bind:checked={value as boolean} />
    {:else if schema.type == "String"}
        <input type="text" bind:value={value} />
    {:else if schema.type == "Number"}
        <input type="number" bind:value={value} />
    {:else if schema.type == "ArrayOf"}
        <div>
            {#each (value as T[]) as item, i}
                <div class="listItem">
                    <span class="entryName">Entry {i + 1}:</span>
                    <div class="entry">
                        <SchemaEditor schema={schema.content} bind:value={(value as T[])[i]} noShell />
                    </div>
                    <div class="controls">
                        {#if i > 0}
                            <button onclick={() => {
                                const entry = (value as T[]).splice(i, 1)[0];
                                (value as T[]).splice(i - 1, 0, entry);
                            }} aria-label="Move up">
                                <i class="fas fa-arrow-up"></i>
                            </button>
                        {/if}
                        {#if i < (value as T[]).length - 1}
                            <button onclick={() => {
                                const entry = (value as T[]).splice(i, 1)[0];
                                (value as T[]).splice(i + 1, 0, entry);
                            }} aria-label="Move down">
                                <i class="fas fa-arrow-down"></i>
                            </button>
                        {/if}
                        <button onclick={() => {
                            const entry = structuredClone($state.snapshot((value as T[])[i]));
                            (value as T[]).splice(i + 1, 0, entry as T);
                        }} aria-label="Duplicate entry">
                            <i class="fas fa-copy"></i>
                        </button>
                        <button onclick={() => {
                            (value as T[]).splice(i, 1);
                        }} aria-label="Remove entry">
                            <i class="fas fa-trash"></i>
                        </button>
                    </div>
                </div>
            {/each}
            <button onclick={() => (value as T[]).push(createDefaultValue(schema.content))}>Add</button>
        </div>
    {:else if schema.type == "Enum"}
        <select value={(value as EnumValue).name} onchange={(e) => {
            const name = (e.target as HTMLSelectElement).value;
            const variant = schema.content.find(v => v.name == name)!;
            if(variant.value != null) {
                (value as EnumValue).value = createDefaultValue(variant.value);
            } else {
                delete (value as EnumValue).value;
            }
            (value as EnumValue).name = name;
        }}>
            {#each schema.content as variant}
                <option value={variant.name}>{camelCaseToReadable(variant.name)}</option>
            {/each}
        </select>
        {#if (value as EnumValue).value != null}
            <SchemaEditor
                schema={schema.content.find(v => v.name == (value as EnumValue).name)!.value!}
                bind:value={(value as EnumValue).value}
                noShell
            />
        {/if}
    {:else if schema.type == "Struct"}
        <div>
            {#each schema.content as field}
                <SchemaEditor schema={field.ty} bind:value={(value as Record<string, unknown>)[field.name]} name={field.name} />
            {/each}
        </div>
    {:else if schema.type == "Optional"}
        <button onclick={() => value = value == null ? createDefaultValue(schema.content) : null}>{value == null ? "Add" : "Remove"}</button>
        {#if value != null}
            <SchemaEditor schema={schema.content} bind:value={value} name={name} />
        {/if}
    {:else if schema.type == "TupleOf"}
        <div>
            {#each schema.content as field, i}
                <SchemaEditor schema={field} bind:value={(value as T[])[i]} name={String(i)} />
            {/each}
        </div>
    {:else if schema.type == "Reference"}
        <!-- Special cases for certain reference types we want editors for -->
        {#if schema.content === "PixelColor"}
            <!-- The value is a struct with r, g, b, and alpha values -->
             <div class="colorPicker">
                <ColorPicker rgb={structColorToRgba(value as PixelColor)} onInput={(color) => {
                    (value as PixelColor) = rgbaToStructColor(color.rgb);
                }} components={ChromeVariant as any} sliderDirection="horizontal" --slider-width="15px" />
            </div>
        {:else}
            <SchemaEditor schema={schemas[schema.content]} bind:value={value} noShell />
        {/if}
    {:else}
        <span>Unknown schema type: {(schema as any).type}</span>
    {/if}
</div>

<style>
    .editor {
        background-color: #252529;
        padding: 0.5rem 1rem;
        font-size: 1.25rem;
        color: white;
    }
    .entryName {
        display: inline;
    }

    input, select, button {
        background-color: #171719;
        color: white;
        border: 1px solid #2a2a2e;
        padding: 0.25rem 0.5rem;
        font-size: 1.25rem;
        margin-top: 0.5rem;
        padding: 0.5rem 1rem;
    }
    button {
        transition: background-color 0.2s, color 0.2s;
    }
    button:hover {
        background-color: #2a2a2e;
        cursor: pointer;
    }

    .colorPicker {
        display: inline;
		--cp-bg-color: #171719;
		--cp-border-color: black;
		--cp-text-color: white;
		--cp-input-color: #252529;
		--cp-button-hover-color: #2a2a2e;
    }

    .listItem {
        display: flex;
        align-items: center;
        margin-bottom: 0.5rem;

        .entryName {
            margin-left: 1rem;
            margin-right: 1rem;
            flex-shrink: 0;
        }
        .entry {
            flex: 1;
        }
    }
</style>