<script lang="ts">
    // import { runEffectPreset } from "../api/presets";
    import type { EffectPreset } from "@shared-bindings/index";
    import { slide } from "svelte/transition";
    import SchemaEditor from "./schemaEditor/SchemaEditor.svelte";
    import { schemas } from "@bindings/schemas";
    import type { AnyEffect } from "@bindings/index";
    import { createEffectPreset, getPresetData } from "../api/presets";

    let { preset }: { preset: EffectPreset } = $props();

    let presetData: AnyEffect | null = $state(null);

    let editing = $state(false);
    // TODO: Preview
    let previewing = $state(false);
    let unsavedChanges = $state(false);

    let previousPresetData = false;
    $effect(() => {
        if(editing) {
            (async () => {
                previousPresetData = false;
                unsavedChanges = false;
                presetData = await getPresetData(preset.name);
            })();
        } else {
            previewing = false;
        }
    });

    $effect(() => {
        console.log("Checking for unsaved changes");
        if(previousPresetData && presetData) unsavedChanges = true;
        previousPresetData = presetData != null;
    });

    function save() {
        if(presetData == null) return;
        
        createEffectPreset(preset.name, preset.icon, presetData);
        unsavedChanges = false;
    }
</script>

<div class="preset">
    <button class={`top ${editing ? "editing" : ""}`} onclick={() => editing = !editing} aria-expanded={editing} aria-label="Toggle preset editing">
        <span>
            <i class={preset.icon}></i>
            {preset.name}
        </span>

        {#if editing}
            <i class="fas fa-chevron-down"></i>
        {:else}
            <i class="fas fa-chevron-right"></i>
        {/if}
    </button>

    {#if editing}
        <div class="edit" transition:slide={{ duration: 300 }}>
            <p>Preset details:</p>
            <ul>
                <li><strong>Name:</strong> {preset.name}</li>
                <li><strong>Icon:</strong> {preset.icon}</li>
            </ul>

            {#if presetData != null}
                <SchemaEditor name="Effect" bind:value={presetData} schema={schemas["AnyEffect"]} />
            {:else}
                <p>Loading...</p>
            {/if}

            <div class="actions">
                <button class="preview" class:enabled={previewing} onclick={() => previewing = !previewing}>
                    {#if previewing}
                        Disable preview
                    {:else}
                        Enable preview
                    {/if}
                </button>
                <button class="save" class:unsavedChanges onclick={save}>Save</button>
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
            color: #eee;
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
            width: 15rem;
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
</style>