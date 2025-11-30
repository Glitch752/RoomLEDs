<script lang="ts">
    import type { MarqueeState, NodeData, SelectionState } from "./NodeTypes";

    let {
        marquee = $bindable()
    }: {
        marquee: MarqueeState
    } = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="marquee-selection-layer"
    onmousedown={(event) => {
        if(event.button !== 0) return; // Only respond to left mouse button

        marquee.active = true;
        const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
        marquee.startX = event.clientX - rect.left;
        marquee.startY = event.clientY - rect.top;
        marquee.endX = marquee.startX;
        marquee.endY = marquee.startY;

        event.preventDefault();
    }}
></div>

<style>
.marquee-selection-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: all;
}
</style>