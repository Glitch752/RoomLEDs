<script lang="ts">
import LightVisualization from "../../lib/LightVisualization.svelte";
import { statusMessage } from "../../websocket";
</script>

<LightVisualization />
<p class="status">{@html $statusMessage}</p>

<style lang="scss">
.status {
  text-align: left;
}
</style>