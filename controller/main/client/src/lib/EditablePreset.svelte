<script lang="ts">
    // import { runEffectPreset } from "../api/presets";
    import type { EffectPreset } from "@shared-bindings/index";
  import { slide } from "svelte/transition";

    let { preset }: { preset: EffectPreset } = $props();

    let editing = $state(false);
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

    .edit {
        font-size: 1.25rem;
        color: #ccc;
        overflow: hidden;
    }
</style>