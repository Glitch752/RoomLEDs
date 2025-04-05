<script module>
    // Pretty hacky solution, but it works
    let previewedComponent = $state<string | null>(null);
</script>

<script lang="ts">
    import type { EffectPreset } from "@shared-bindings/index";
    import { slide } from "svelte/transition";
    import SchemaEditor from "./schemaEditor/SchemaEditor.svelte";
    import { schemas } from "@bindings/schemas";
    import type { AnyEffect } from "@bindings/index";
    import { createEffectPreset, deleteEffectPreset, getPresetData, runArbitraryEffect } from "../api/presets";
    import { debounce } from "../util/debouncer";

    let { preset }: { preset: EffectPreset } = $props();

    const id = $props.id();
    let debounceEffectUpdate = debounce(0.25);

    let presetData: AnyEffect | null = $state(null);

    let editing = $state(false);
    // TODO: Preview
    let previewing = $derived(id === previewedComponent);
    let unsavedChanges = $state(false);

    function togglePreview() {
        if(!presetData) return;

        if(previewing) {
            previewedComponent = null;
            previewing = false;
            runArbitraryEffect(null);
        } else {
            previewedComponent = id;
            previewing = true;
            runArbitraryEffect(presetData);
        }
    }

    let previousPresetData = false;
    async function swapEditing() {
        if(!editing && !unsavedChanges) {
            previousPresetData = false;
            unsavedChanges = false;
            presetData = await getPresetData(preset.name);
        }
        editing = !editing;
    }

    function onchange() {
        if(previousPresetData && presetData) unsavedChanges = true;
        previousPresetData = presetData != null;

        if(previewing) {
            debounceEffectUpdate(() => runArbitraryEffect(presetData));
        }
    }

    $effect(() => {
        if(presetData) onchange();
    });

    function save() {
        if(presetData == null) return;
        
        createEffectPreset(preset.name, preset.icon, presetData);
        unsavedChanges = false;
    }

    function deleteEffect() {
        if(!confirm("Are you sure you want to delete this effect?")) return;

        if(presetData) {
            runArbitraryEffect(null);
            deleteEffectPreset(preset.name);
        }
    }
</script>

<div class="preset">
    <button class={`top ${editing ? "editing" : ""}`} onclick={swapEditing} aria-expanded={editing} aria-label="Toggle preset editing">
        <span class="name">
            <i class={preset.icon}></i>
            {#if unsavedChanges}
                <span>*</span>
            {/if}
            {preset.name}
        </span>

        {#if previewing}
            <span class="tag previewing">Previewing</span>
        {/if}
        {#if editing}
            <i class="fas fa-chevron-down"></i>
        {:else}
            <i class="fas fa-chevron-right"></i>
        {/if}
    </button>

    {#if editing}
        <div class="edit" transition:slide={{ duration: 300 }}>

            {#if presetData != null}
                <SchemaEditor name="Effect" bind:value={presetData} schema={schemas["AnyEffect"]} 
                {onchange} />
            {:else}
                <p>Loading...</p>
            {/if}

            <div class="actions">
                <button class="preview" class:enabled={previewing} onclick={togglePreview}>
                    {#if previewing}
                        Disable preview
                    {:else}
                        Enable preview
                    {/if}
                </button>
                <button class="save" class:unsavedChanges onclick={save}>Save</button>
                <button class="close" onclick={swapEditing}>Close</button>
                <button class="delete" onclick={deleteEffect}>Delete</button>
            </div>
        </div>
    {/if}
</div>

<style lang="scss">
    .preset {
        width: 100%;
        background-color: #252529;
        margin: 0.5rem 0;
        text-align: left;
    }

    .top {
        display: flex;
        border: none;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        background: none;
        color: #ccc;
        font-size: 1.75rem;
        transition: background-color 0.2s, color 0.2s;
        height: 3rem;

        i {
            width: 3.5rem;
            text-align: center;
            color: #eee;
        }
        .name {
            flex-grow: 1;
            text-align: left;
        }

        .tag {
            font-size: 1.25rem;
            padding: 0.25rem 1rem;
            margin-left: 1rem;
            background-color: #2a2a2e;
            color: white;

            &.previewing {
                background-color: #2a406a;
                color: white;
            }
        }

        &.editing {
            background-color: #2a2a2e;
            color: white;
        }
    }

    p {
        margin: 0;
        padding: 0;
    }
    
    .edit {
        font-size: 1.25rem;
        padding: 2rem;
        color: #ccc;
        overflow: hidden;
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

    button.save {
        background-color: #3e403f;
    }
    button.save:hover {
        background-color: #4e504f;
    }
    button.save:active {
        background-color: #2e3030;
    }

    button.save.unsavedChanges {
        background-color: #406a2a;
    }
    button.save.unsavedChanges:hover {
        background-color: #4c7a2a;
    }
    button.save.unsavedChanges:active {
        background-color: #3c5a2a;
    }

    button.preview {
        background-color: #2a406a;
    }
    button.preview:hover {
        background-color: #2a4c7a;
    }
    button.preview:active {
        background-color: #2a3c5a;
    }

    button.preview.enabled {
        background-color: #6a2a40;
    }
    button.preview.enabled:hover {
        background-color: #7a2a4c;
    }
    button.preview.enabled:active {
        background-color: #5a2a3c;
    }

    button.close {
        background-color: #3e403f;
    }
    button.close:hover {
        background-color: #4e504f;
    }
    button.close:active {
        background-color: #2e3030;
    }

    button.delete {
        background-color: #6a2a2a;
    }
    button.delete:hover {
        background-color: #7a2a2a;
    }
    button.delete:active {
        background-color: #5a2a2a;
    }
</style>