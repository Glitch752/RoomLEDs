<script lang="ts">
    import { runEffectPreset } from "../../api/presets";
    import EffectCreator from "../../lib/EffectCreator.svelte";
    import { presets } from "../../websocket";
    
    let creatingPreset = $state(false);
</script>
  
<div class="content">
    <h1>Presets</h1>
    {#each $presets as preset}
        <button class="preset" title={preset.name} onclick={() => runEffectPreset(preset.name)} aria-label={preset.name}>
            <i class={preset.icon}></i>
            {preset.name}
        </button>
    {/each}

    <button class="preset" title="Create new preset" onclick={() => creatingPreset = true} aria-label="Create new preset">
        <i class="fas fa-plus"></i>
    </button>
    
    {#if creatingPreset}
        <EffectCreator onClose={() => creatingPreset = false} />
    {/if}
</div>
  
<style lang="scss">
    h1 {
        margin: 0;
        padding: 0;
    }
    .content {
        padding: 2rem;
        overflow-y: auto;
    }
    .preset {
        width: 100%;
        background-color: #252529;
        border: none;
        font-size: 2rem;
        color: #ccc;
        cursor: pointer;
        margin: 0.5rem 0;
        padding: 0.5rem 1rem;
        text-align: left;

        i {
            width: 3.5rem;
            color: #eee;
        }
    }
</style>