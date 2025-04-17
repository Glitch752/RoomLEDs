<script lang="ts">
import type { EnumValue as EnumSchema } from "@bindings/schema_types";
import SchemaEditor from "./SchemaEditor.svelte";
import { camelCaseToReadable, createDefaultValue } from "./schemaUtils.svelte";

type EnumValue = {
    [value: string]: any
};

let {
    schema,
    value = $bindable(),
    onchange
}: {
    schema: EnumSchema,
    value: EnumValue,
    onchange?: () => void
} = $props();

function setValue(content: any, name: string) {
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
    const variant = schema.variants.find(v => v.name == name);
    if(!variant) return;

    if(variant.value != null) {
        setValue(createDefaultValue(variant.value), name);
    } else {
        value = {
            [schema.tag_name]: variant.name
        };
    }

    onchange?.();
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
            {onchange}
        />
    {:else}
        <SchemaEditor
            schema={schema.variants.find(v => v.name == value[schema.tag_name])!.value!}
            bind:value={value}
            noShell
            {onchange}
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