<script lang="ts">
    import type { AnyEffect } from "@bindings/index";
    import SchemaEditor from "./schemaEditor/SchemaEditor.svelte";
    import { schemas } from "@bindings/schemas";

    let { onclose }: { onclose: () => void } = $props();

    const effectTypes: string[] = [];
    let effect: AnyEffect = $state({
        type: "SolidColor",
        color: {
            r: 1,
            g: 1,
            b: 1,
            alpha: 1
        },
        start: 0,
        stop: 814
    });
</script>

<div>
    <SchemaEditor name="Effect" bind:value={effect} schema={schemas["AnyEffect"]} />
    <button onclick={onclose}>Cancel</button>
</div>

<style>

</style>