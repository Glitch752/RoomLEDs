<script lang="ts">
  import { slide } from "svelte/transition";
    import EditablePreset from "../../lib/EditablePreset.svelte";
    import EffectCreator from "../../lib/EffectCreator.svelte";
    import { presets } from "../../websocket";
    
    let creatingPreset = $state(false);
</script>
  
<div class="content">
    {#each $presets as preset}
        <EditablePreset id={preset.id} name={preset.name} icon={preset.icon} />
    {/each}

    {#if creatingPreset}
        <div transition:slide={{ duration: 300 }}>
            <EffectCreator onclose={() => creatingPreset = false}/>
        </div>
    {/if}
    
    <button class="addPreset gray" title="Create new preset" onclick={() => creatingPreset = true} aria-label="Create new preset">
        <i class="fas fa-plus"></i>
    </button>
</div>
  
<style lang="scss">
    .content {
        padding: 0.25rem 1rem 1rem 1rem;
        overflow-y: auto;
        height: 100%;
    }

    .addPreset {
        width: 100%;
        background-color: var(--surface0);
        border: none;
        font-size: 1.75rem;
        color: var(--text);
        cursor: pointer;
        padding: 0.5rem 1rem;
        text-align: center;
    }
</style>