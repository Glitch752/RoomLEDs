<script lang="ts" generics="T">
import type { Schema } from '@bindings/schema_types';
import SchemaEditor from './SchemaEditor.svelte';
import { createDefaultValue } from './schemaEditor.svelte';

let {
    schema,
    value = $bindable(),
    onchange
}: {
    schema: Schema,
    onchange?: () => void,
    value: T[]
} = $props();
</script>

<div>
    {#each value as item, i}
        <div class="listItem">
            <span class="entryName">Item {i + 1}:</span>
            <div class="entry">
                <SchemaEditor schema={schema} bind:value={value[i]} {onchange} />
            </div>
            <div class="controls">
                {#if i > 0}
                    <button onclick={() => {
                        const entry = value.splice(i, 1)[0];
                        value.splice(i - 1, 0, entry);
                        onchange?.();
                    }} aria-label="Move up">
                        <i class="fas fa-arrow-up"></i>
                    </button>
                {/if}
                {#if i < value.length - 1}
                    <button onclick={() => {
                        const entry = value.splice(i, 1)[0];
                        value.splice(i + 1, 0, entry);
                        onchange?.();
                    }} aria-label="Move down">
                        <i class="fas fa-arrow-down"></i>
                    </button>
                {/if}
                <button onclick={() => {
                    const entry = structuredClone($state.snapshot(value[i]));
                    value.splice(i + 1, 0, entry as T);
                    onchange?.();
                }} aria-label="Duplicate entry">
                    <i class="fas fa-copy"></i>
                </button>
                <button onclick={() => {
                    value.splice(i, 1);
                    onchange?.();
                }} aria-label="Remove entry">
                    <i class="fas fa-trash"></i>
                </button>
            </div>
        </div>
    {/each}
    <button onclick={() => {
        console.log(value);
        value.push(createDefaultValue(schema));
        onchange?.();
    }}>Add</button>
</div>

<style>
.listItem {
    display: flex;
    align-items: center;

    .entryName {
        margin: 0 0.5rem;
        flex-shrink: 0;
    }
    .entry {
        flex: 1;
    }
}

button {
    background-color: #171719;
    color: white;
    border: 1px solid #2a2a2e;
    padding: 0.25rem 0.5rem;
    font-size: 1.25rem;
    padding: 0.5rem 1rem;
}
button {
    transition: background-color 0.2s, color 0.2s;
}
button:hover {
    background-color: #2a2a2e;
    cursor: pointer;
}

.controls button {
    font-size: 1rem;
}
</style>