<script lang="ts">
import type { EnumValue as EnumSchema } from "@bindings/schema_types";
import SchemaEditor from "./SchemaEditor.svelte";
import { camelCaseToReadable, createDefaultValue } from "./schemaEditor.svelte";

type EnumValue = {
    [value: string]: any
};

let {
    schema,
    value = $bindable(),
    /**
     * Called when a primitive value (not an object or array) is changed.  
     * Used to fix deep reactivity not working with nested objects in this way.
    */
    onPrimitiveChange = (value: EnumValue) => {}
}: {
    schema: EnumSchema,
    value: EnumValue,
    onPrimitiveChange?: (value: EnumValue) => void
} = $props();

// This is probably super inefficient, but... oh well. It works.
function updateState(newValue: any) {
    onPrimitiveChange(newValue);
    value = { ...value };
}


function setValue(content: any) {
    if(schema.content_subfield != null) {
        value = {
            [schema.tag_name]: name,
            [schema.content_subfield]: content
        };
    } else {
        value = {
            [schema.tag_name]: name
        };
        if(typeof content == "object") {
            // Copy all fields from the value object to the enum value
            for(const key in content) {
                value[key] = content[key];
            }
        } else {
            // Weird unsupported case
            throw new Error("Unsupported case");
        }
    }
}
</script>

<select value={value[schema.tag_name]} onchange={(e) => {
    const name = (e.target as HTMLSelectElement).value;
    const variant = schema.variants.find(v => v.name == name)!;

    if(variant.value != null) {
        setValue(createDefaultValue(variant.value));
    } else {
        value = {
            [schema.tag_name]: variant.name
        };
    }

    onPrimitiveChange(value);
}}>
    {#each schema.variants as variant}
        <option value={variant.name}>{camelCaseToReadable(variant.name)}</option>
    {/each}
</select>
{#if schema.variants.find(v => v.name == value[schema.tag_name])?.value != null}
    {#if schema.content_subfield != null}
        <SchemaEditor
            schema={schema.variants.find(v => v.name == value[schema.tag_name])!.value!}
            bind:value={value[schema.content_subfield!]}
            noShell
            onPrimitiveChange={updateState}
        />
    {:else}
        <SchemaEditor
            schema={schema.variants.find(v => v.name == value[schema.tag_name])!.value!}
            bind:value={value}
            noShell
            onPrimitiveChange={updateState}
        />
    {/if}
{/if}

<style>
    select {
        background-color: #171719;
        color: white;
        border: 1px solid #2a2a2e;
        padding: 0.25rem 0.5rem;
        font-size: 1.25rem;
        padding: 0.5rem 1rem;
    }
</style>