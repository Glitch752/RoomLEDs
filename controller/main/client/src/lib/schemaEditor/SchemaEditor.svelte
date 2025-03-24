<script lang="ts">
import type { Schema, SchemaField } from "@bindings/schema_types";
import { schemas } from "@bindings/schemas";
import SchemaEditor from "./SchemaEditor.svelte";

type T = $$Generic;
// Yuck... what is this runes syntax (I'm just biased)
let { schema, value = $bindable(), name }: { schema: Schema, value: T, name: string } = $props();

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
            return createDefaultValue(schemas[schema.content]);
    }
}
</script>

<div>
    <span>{name}</span>
    {#if schema.type == "Boolean"}
        <input type="checkbox" bind:checked={value as boolean} />
    {:else if schema.type == "String"}
        <input type="text" bind:value={value} />
    {:else if schema.type == "Number"}
        <input type="number" bind:value={value} />
    {:else if schema.type == "ArrayOf"}
        <div>
            {#each (value as T[]) as item, i}
                <SchemaEditor schema={schema.content} value={item} name={String(i)} />
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
                <option value={variant.name}>{variant.name}</option>
            {/each}
        </select>
        {#if (value as EnumValue).value != null}
            <SchemaEditor
                schema={schema.content.find(v => v.name == (value as EnumValue).name)!.value!}
                value={(value as EnumValue).value}
                name={name}
            />
        {/if}
    {:else if schema.type == "Struct"}
        <div>
            {#each schema.content as field}
                <SchemaEditor schema={field.ty} value={(value as Record<string, unknown>)[field.name] ?? createDefaultValue(field.ty)} name={field.name} />
            {/each}
        </div>
    {:else if schema.type == "Optional"}
        <button onclick={() => value = value == null ? createDefaultValue(schema.content) : null}>{value == null ? "Add" : "Remove"}</button>
        {#if value != null}
            <SchemaEditor schema={schema.content} value={value} name={name} />
        {/if}
    {:else if schema.type == "TupleOf"}
        <div>
            {#each schema.content as field, i}
                <SchemaEditor schema={field} value={(value as T[])[i] ?? createDefaultValue(field)} name={String(i)} />
            {/each}
        </div>
    {:else if schema.type == "Reference"}
        <SchemaEditor schema={schemas[schema.content]} value={value} name={name} />
    {:else}
        <span>Unknown schema type: {(schema as any).type}</span>
    {/if}
</div>

<style>

</style>