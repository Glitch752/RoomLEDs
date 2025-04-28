<script lang="ts">
    import type { AnyEffect } from "@bindings/index";
    import SchemaEditor from "./schemaEditor/SchemaEditor.svelte";
    import { schemas } from "@bindings/schemas";
    import { createEffectPreset, runArbitraryEffect } from "../api/presets";
    import IconSelector from "./iconSelector/IconSelector.svelte";
    import { previewedComponent, setPreviewedComponent } from "./preview.svelte";
    import { debounce } from "../util/debouncer";
    
    const id = $props.id();
    let debounceEffectUpdate = debounce(0.25);
    
    let previewing = $derived(id === previewedComponent.id);
    function togglePreview() {
        if(previewing) {
            setPreviewedComponent(null);
            previewing = false;
            runArbitraryEffect(null);
        } else {
            setPreviewedComponent(id);
            previewing = true;
            runArbitraryEffect(effect);
        }
    }

    function onchange() {
        if(previewing) {
            debounceEffectUpdate(() => runArbitraryEffect(effect));
        }
    }

    let { onclose }: { onclose: () => void } = $props();

    let name = $state("New effect");
    let icon = $state("fas fa-circle");

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
    
    <h3>Name</h3>
    <input bind:value={name} placeholder="Name" aria-label="Name" />

    <h3>Icon</h3>
    <IconSelector bind:value={icon} placeholder="Icon" ariaLabel="Icon" />

    <h3>Effect</h3>
    <SchemaEditor {onchange} bind:value={effect} schema={schemas["AnyEffect"]} />

    <div class="actions">
        <button class:green={!previewing} class:peach={previewing} onclick={togglePreview}>
            {#if previewing}    
                <i class="fas fa-eye-slash"></i>
                Disable preview
            {:else}
                <i class="fas fa-eye"></i>
                Enable preview
            {/if}
        </button>
        <button class="gray" onclick={onclose}>Cancel</button>
        <button class="green" onclick={() => {
            createEffectPreset(name, icon, effect);
            onclose();
        }}>Create</button>
    </div>
</div>

<style>
.editor {
    width: 100%;
    background-color: var(--surface0);
    margin: 0.5rem 0;
    color: var(--text);
    padding: 1rem;
    text-align: left;
}

h2 {
    font-size: 2rem;
    margin: 0 0 1rem 0;
}
h3 {
    font-size: 1.5rem;
    margin: 1.5rem 0 0 0;
    font-weight: 500;
}

.actions {
    display: flex;
    justify-content: flex-start;
    margin-top: 1rem;
    gap: 1rem;
    font-size: 1rem;
}
</style>