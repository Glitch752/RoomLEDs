<script lang="ts">
  import LightVisualization from "../lib/LightVisualization.svelte";
  import { statusMessage } from "../websocket";
</script>

<main>
  <h1>Full interface</h1>
  <LightVisualization />
  <p class="status">{@html $statusMessage}</p>
</main>

<style>
.status {
  text-align: left;
}
</style>
