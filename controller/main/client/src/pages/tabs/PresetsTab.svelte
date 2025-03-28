<script lang="ts">
    import EditablePreset from "../../lib/EditablePreset.svelte";
    import EffectCreator from "../../lib/EffectCreator.svelte";
    import { presets } from "../../websocket";
    
    let creatingPreset = $state(false);
</script>
  
<div class="content">
    <h1>Presets</h1>
    {#each $presets as preset}
        <EditablePreset preset={preset} />
    {/each}

    {#if creatingPreset}
        <EffectCreator onclose={() => creatingPreset = false} />
    {/if}
    
    <button class="addPreset" title="Create new preset" onclick={() => creatingPreset = true} aria-label="Create new preset">
        <i class="fas fa-plus"></i>
    </button>
</div>
  
<style lang="scss">
    h1 {
        margin: 0;
        padding: 0;
    }
    .content {
        padding: 2rem;
        overflow-y: auto;
        height: 100%;
    }

    .addPreset {
        width: 100%;
        background-color: #252529;
        border: none;
        font-size: 1.75rem;
        color: white;
        cursor: pointer;
        padding: 0.5rem 1rem;
        text-align: center;
    }
</style>