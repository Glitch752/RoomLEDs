<script lang="ts">
    // import { runEffectPreset } from "../api/presets";
    import type { EffectPreset } from "@shared-bindings/index";

    let { preset }: { preset: EffectPreset } = $props();

    let editing = $state(false);
</script>

<div class={`preset ${editing ? "editing" : ""}`}>
    <button class="top" onclick={() => editing = !editing} aria-expanded={editing} aria-label="Toggle preset editing">
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

    <div class="edit">
        <p>Preset details:</p>
        <ul>
            <li><strong>Name:</strong> {preset.name}</li>
            <li><strong>Icon:</strong> {preset.icon}</li>
        </ul>
    </div>
</div>

<style lang="scss">
    .preset {
        width: 100%;
        background-color: #252529;
        margin: 0.5rem 0;
        text-align: left;

        display: grid;
        grid-template-rows: 3rem 0fr;
        transition: grid-template-rows 0.2s ease;

        &.editing {
            grid-template-rows: 3rem 1fr;
        }
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

        i {
            width: 3.5rem;
            color: #eee;
        }
    }

    .editing .top {
        background-color: #2a2a2e;
        color: white;
    }

    .edit {
        font-size: 1.25rem;
        color: #ccc;
        overflow: hidden;
    }
</style>