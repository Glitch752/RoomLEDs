<script lang="ts">
    import { runEffectPreset } from "../../api/presets";
    import LightVisualization from "../../lib/LightVisualization.svelte";
    import { presets } from "../../websocket";
</script>

<div class="content">
    <h1>Presets</h1>
    {#each $presets as preset}
        <button title={preset.name} onclick={() => runEffectPreset(preset.name)} aria-label={preset.name}>
            <i class={preset.icon}></i>
        </button>
    {/each}
    
    <br />
    <br />
    <LightVisualization />
</div>
    
<style lang="scss">
    h1 {
        margin: 0;
        padding: 0;
        text-align: left;
        font-size: 1.5rem;
        font-weight: 600;
    }

    .content {
        padding: 1rem 3rem 3rem 3rem;
        overflow-y: auto;
    }
    .status {
        text-align: left;
    }
    
    button {
        background: none;
        border: none;
        font-size: 2rem;
        color: var(--text);
        cursor: pointer;
        padding: 0.5rem;
        margin: 0.5rem;
        transition: color 0.2s, transform 0.2s ease;
    }
    button:hover {
        color: var(--subtext1);
        transform: translateY(-2px);
    }
</style>