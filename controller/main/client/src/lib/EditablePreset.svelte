<script lang="ts">
    import type { EffectPreset } from "@shared-bindings/index";
    import { fade, slide } from "svelte/transition";
    import SchemaEditor from "./schemaEditor/SchemaEditor.svelte";
    import { schemas } from "@bindings/schemas";
    import type { AnyEffect } from "@bindings/index";
    import { createEffectPreset, deleteEffectPreset, getPresetData, runArbitraryEffect } from "../api/presets";
    import { debounce } from "../util/debouncer";
    import IconSelector from "./iconSelector/IconSelector.svelte";
    import { previewedComponent, setPreviewedComponent } from "./preview.svelte";

    let { name: defaultName, icon: defaultIcon }: { name: string, icon: string } = $props();

    const id = $props.id();
    let debounceEffectUpdate = debounce(0.25);

    let presetData: AnyEffect | null = $state(null);
    let name: string = $state(defaultName);
    let icon: string = $state(defaultIcon);

    // On the name or icon changing, we need to delete and recreate the effect preset
    // TODO: We should store effects by ID instead of name, but this is a quick fix
    let currentServerName = defaultName;
    let currentServerIcon = defaultIcon;
    $effect(() => {
        if(!presetData) return;
        if(name !== currentServerName || icon !== currentServerIcon) {
            createEffectPreset(name, icon, presetData);
            deleteEffectPreset(currentServerName);
            currentServerName = name;
            currentServerIcon = icon;
        }
    });

    let editing = $state(false);
    let previewing = $derived(id === previewedComponent.id);
    let unsavedChanges = $state(false);

    function togglePreview() {
        if(!presetData) return;

        if(previewing) {
            setPreviewedComponent(null);
            previewing = false;
            runArbitraryEffect(null);
        } else {
            setPreviewedComponent(id);
            previewing = true;
            runArbitraryEffect(presetData);
        }
    }

    let previousPresetData = false;
    async function swapEditing() {
        if(!editing && !unsavedChanges) {
            previousPresetData = false;
            unsavedChanges = false;
            presetData = await getPresetData(name);
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
        
        createEffectPreset(name, icon, presetData);
        unsavedChanges = false;
    }

    function deleteEffect() {
        if(!confirm("Are you sure you want to delete this effect?")) return;

        if(presetData) {
            runArbitraryEffect(null);
            deleteEffectPreset(name);
        }
    }
</script>

<div class="preset">
    <button class={`top ${editing ? "editing" : ""}`} onclick={swapEditing} aria-expanded={editing} aria-label="Toggle preset editing">
        <span class="name">
            <i class={icon}></i>
            {#if unsavedChanges}
                <span>*</span>
            {/if}
            {name}
        </span>

        {#if previewing}
            <span class="tag previewing" transition:fade={{ duration: 100 }}>Previewing</span>
        {/if}
        {#if editing}
            <i class="fas fa-chevron-down"></i>
        {:else}
            <i class="fas fa-chevron-right"></i>
        {/if}
    </button>

    {#if editing}
        <div class="edit" transition:slide={{ duration: 300 }}>
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
                <button class:gray={!unsavedChanges} class:green={unsavedChanges} onclick={save}>
                    <i class="fas fa-floppy-disk"></i>
                    Save
                </button>
                <button class="red" onclick={deleteEffect}><i class="fas fa-trash-can"></i>Delete</button>
            </div>
            
            <IconSelector bind:value={icon} placeholder="Icon" ariaLabel="Icon" />

            {#if presetData != null}
                <SchemaEditor bind:value={presetData} schema={schemas["AnyEffect"]} 
                {onchange} />
            {:else}
                <p>Loading...</p>
            {/if}

            <div class="actions">
                <button class="gray" onclick={swapEditing}>Close</button>
            </div>
        </div>
    {/if}
</div>

<style lang="scss">
    .preset {
        width: 100%;
        background-color: var(--surface0);
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
        color: var(--subtext0);
        font-size: 1.5rem;
        transition: background-color 0.2s, color 0.2s;
        height: 3rem;
        padding: 0.5rem;

        i {
            width: 2.5rem;
            text-align: center;
            color: var(--subtext0);
        }
        .name {
            flex-grow: 1;
            text-align: left;
        }

        .tag {
            font-size: 1.25rem;
            padding: 0.25rem 1rem;
            margin-left: 1rem;

            &.previewing {
                background-color: var(--sapphire);
                color: var(--surface0);
            }
        }

        &.editing {
            background-color: var(--surface1);
            color: var(--text);
        }
    }

    button > i {
        margin-right: 0.5rem;
    }

    p {
        margin: 0;
        padding: 0;
    }
    
    .edit {
        font-size: 1.25rem;
        padding: 1rem;
        color: var(--text);
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .actions {
        display: flex;
        justify-content: flex-start;
        font-size: 1rem;
        gap: 0.5rem;
    }
</style>