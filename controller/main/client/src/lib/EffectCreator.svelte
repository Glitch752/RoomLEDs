<script lang="ts">
    import type { AnyEffect } from "@bindings/index";
    import SchemaEditor from "./schemaEditor/SchemaEditor.svelte";
    import { schemas } from "@bindings/schemas";
    import { createEffectPreset, getEffectPresets } from "../api/presets";
  import IconSelector from "./iconSelector/IconSelector.svelte";

    let { onclose }: { onclose: () => void } = $props();

    let name = $state("New effect");
    let icon = $state("fas fa-circle");

    const effectTypes: string[] = [];
    let effect: AnyEffect = $state({
        type: "SolidColor",
        color: {
            r: 1,
            g: 1,
            b: 1,
            alpha: 1
        },
        start: 0,
        stop: 814
    });
</script>

<div class="editor">
    <h2>New effect</h2>
    
    <span class="entryName">Name</span>
    <input bind:value={name} placeholder="Name" aria-label="Name" />

    <IconSelector bind:value={icon} placeholder="Icon" ariaLabel="Icon" />

    <SchemaEditor name="Effect" bind:value={effect} schema={schemas["AnyEffect"]} />
    <div class="actions">
        <button class="cancel" onclick={onclose}>Cancel</button>
        <button class="create" onclick={() => {
            createEffectPreset(name, icon, effect);
            onclose();
        }}>Create</button>
    </div>
</div>

<style>
.editor {
    width: 100%;
    background-color: #252529;
    margin: 0.5rem 0;
    color: white;
    padding: 1rem;
    text-align: left;
}

h2 {
    font-size: 2rem;
    margin: 0 0 1rem 0;
}

.actions {
    display: flex;
    justify-content: flex-start;
    margin-top: 2rem;

    button {
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        margin: 0 0.5rem;
        cursor: pointer;
        font-size: 1.25rem;
    }
}
</style>