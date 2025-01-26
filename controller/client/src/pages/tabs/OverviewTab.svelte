<script lang="ts">
import LightVisualization from "../../lib/LightVisualization.svelte";
import { statusMessage } from "../../websocket";
</script>

<div class="content">
  <LightVisualization />
  <p class="status">{@html $statusMessage}</p>
</div>

<style lang="scss">
.content {
  padding: 3rem;
}
.status {
  text-align: left;
}
</style>