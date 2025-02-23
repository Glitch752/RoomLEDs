<script lang="ts">
  import { runEffectPreset } from "../../api/presets";
  import EffectCreator from "../../lib/EffectCreator.svelte";
import LightVisualization from "../../lib/LightVisualization.svelte";
import { presets, statusMessage } from "../../websocket";

let creatingPreset = $state(false);
</script>

<div class="content">
  <LightVisualization />
  
  <br />
  <h1>Presets</h1>
  {#each $presets as preset}
    <button title={preset.name} onclick={() => runEffectPreset(preset.name)} aria-label={preset.name}>
      <i class={preset.icon}></i>
    </button>
  {/each}
  <button title="Create new preset" onclick={() => creatingPreset = true} aria-label="Create new preset">
    <i class="fas fa-plus"></i>
  </button>
  
  {#if creatingPreset}
    <EffectCreator onClose={() => creatingPreset = false} />
  {/if}

  <br />
  
  <p class="status">{@html $statusMessage}</p>
</div>

<style lang="scss">
.content {
  padding: 3rem;
  overflow-y: auto;
}
.status {
  text-align: left;
}

button {
  background: none;
  border: none;
  font-size: 2rem;
  color: #ccc;
  cursor: pointer;
  padding: 0.5rem;
  margin: 0.5rem;
  transition: color 0.2s;
}
button:hover {
  color: white;
}
</style>