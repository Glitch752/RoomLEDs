<script lang="ts" generics="T">
import type { Schema } from "@bindings/schema_types";
import type { PixelColor } from "@bindings/index";
import { schemas } from "@bindings/schemas";
import SchemaEditor from "./SchemaEditor.svelte";
import ColorPicker, { ChromeVariant, type RgbaColor } from 'svelte-awesome-color-picker';
import EnumSchemaEditor from "./EnumSchemaEditor.svelte";
import { createDefaultValue, snakeCaseToReadable } from "./schemaUtils.svelte";
import ArraySchemaEditor from "./ArraySchemaEditor.svelte";

let {
    schema,
    value = $bindable(),
    name = "",
    onchange,
    noShell = false
}: {
    schema: Schema,
    value: T,
    name?: string,
    onchange?: () => void,
    noShell?: boolean
} = $props();

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
</script>

{#if !noShell && name !== ""}
<span class="entryName">{snakeCaseToReadable(name)}</span>
{/if}
<div class:shell={!noShell}>
    {#if schema.type == "Boolean"}
        <input type="checkbox" bind:checked={value as boolean} {onchange} />
    {:else if schema.type == "String"}
        <input type="text" bind:value={value} {onchange} />
    {:else if schema.type == "Number"}
        <input type="number" bind:value={value} {onchange} />
    {:else if schema.type == "ArrayOf"}
        <ArraySchemaEditor schema={schema.content} bind:value={value as T[]} {onchange} />
    {:else if schema.type == "Enum"}
        <EnumSchemaEditor schema={schema.content} bind:value={value as any} {onchange} />
    {:else if schema.type == "Struct"}
        <div>
            {#each schema.content as field}
                <SchemaEditor schema={field.ty} bind:value={(value as Record<string, unknown>)[field.name]} name={field.name} {onchange} />
            {/each}
        </div>
    {:else if schema.type == "Optional"}
        <button onclick={() => value = value == null ? createDefaultValue(schema.content) : null}>{value == null ? "Add" : "Remove"}</button>
        {#if value != null}
            <SchemaEditor schema={schema.content} bind:value={value} name={name} {onchange} />
        {/if}
    {:else if schema.type == "TupleOf"}
        <div>
            {#each schema.content as field, i}
                <SchemaEditor schema={field} bind:value={(value as T[])[i]} name={String(i)} {onchange} />
            {/each}
        </div>
    {:else if schema.type == "Reference"}
        <!-- Special cases for certain reference types we want editors for -->
        {#if schema.content === "PixelColor"}
            <!-- The value is a struct with r, g, b, and alpha values -->
             <div class="colorPicker">
                <ColorPicker rgb={structColorToRgba(value as PixelColor)} onInput={(color) => {
                    if(
                        color.rgb?.r === (value as PixelColor).r &&
                        color.rgb?.g === (value as PixelColor).g &&
                        color.rgb?.b === (value as PixelColor).b &&
                        color.rgb?.a === (value as PixelColor).alpha
                    ) return;
                    (value as PixelColor) = rgbaToStructColor(color.rgb);
                    onchange?.();
                }} components={ChromeVariant as any} sliderDirection="horizontal" --slider-width="15px"/>
            </div>
        {:else}
            <SchemaEditor schema={schemas[schema.content]} bind:value={value} noShell {onchange} />
        {/if}
    {:else}
        <span>Unknown schema type: {(schema as any).type}</span>
    {/if}
</div>

<style>
    .shell {
        background-color: var(--surface0);
        padding: 0 0.5rem;
        font-size: 1.25rem;
        color: var(--text);
        margin: 0.5rem 0 0.5rem 0.75rem;
        border-left: 4px solid var(--background);
    }

    .entryName {
        display: inline;
    }

    input, button {
        background-color: var(--background);
        color: var(--text);
        border: none;
        padding: 0.25rem 1rem;
        font-size: 1.25rem;
    }
    button:hover {
        background-color: var(--dark-bg);
    }

    .colorPicker {
        display: inline;
		--cp-bg-color: var(--surface0);
		--cp-border-color: var(--contrast-border);
		--cp-text-color: var(--text);
		--cp-input-color: var(--surface1);
		--cp-button-hover-color: var(--surface2);
    }
</style>