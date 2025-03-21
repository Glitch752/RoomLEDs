<script lang="ts">
    import type { AnyEffect } from "@bindings/index";

    let { onClose }: { onClose: () => void } = $props();

    const effectTypes: string[] = [];
</script>

<div>
    <p>Todo</p>
    <button onclick={onClose}>Cancel</button>
</div>

<style>

</style>