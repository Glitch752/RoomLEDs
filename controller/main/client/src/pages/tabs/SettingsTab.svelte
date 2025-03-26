<script lang="ts">
    import { statusMessage } from "../../websocket";
</script>

<div class="content">
    <h1>TODO: Settings</h1>
    <p class="status">{@html $statusMessage}</p>
</div>

<style lang="scss">
.content {
    text-align: left;
}
</style>