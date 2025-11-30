<script lang="ts">
    import type { MarqueeState, NodeData, SelectionState } from "./NodeTypes";

    let {
        marquee = $bindable(),
        selection = $bindable()
    }: {
        marquee: MarqueeState,
        selection: SelectionState
    } = $props();

    $inspect(selection);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
{#if marquee.active}
    <div class="marquee-selection-layer"
        onmousemove={(event) => {
            if(!marquee.active) return;

            const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
            marquee.endX = event.clientX - rect.left;
            marquee.endY = event.clientY - rect.top;

            event.preventDefault();
        }}

        onmouseup={(event) => {
            if(event.button !== 0 || !marquee.active) return; // Only respond to left mouse button

            marquee.active = false;

            const x1 = Math.min(marquee.startX, marquee.endX);
            const y1 = Math.min(marquee.startY, marquee.endY);
            const x2 = Math.max(marquee.startX, marquee.endX);
            const y2 = Math.max(marquee.startY, marquee.endY);

            const oldActiveNode = selection.activeNode;
            selection.activeNode = null;
            selection.nodes.clear();

            const nodes = (event.target as HTMLDivElement)
                .parentElement?.querySelectorAll('[data-node-id]') ?? [];
            for(const node of nodes) {
                const nodeId = node.getAttribute('data-node-id');
                if(!nodeId) continue;

                const nodeRect = node.getBoundingClientRect();
                const parentRect = (event.target as HTMLDivElement).getBoundingClientRect();

                const nodeX1 = nodeRect.left - parentRect.left;
                const nodeY1 = nodeRect.top - parentRect.top;
                const nodeX2 = nodeX1 + nodeRect.width;
                const nodeY2 = nodeY1 + nodeRect.height;

                // Check if the node is within the marquee selection
                if(nodeX1 < x2 && nodeX2 > x1 && nodeY1 < y2 && nodeY2 > y1) {
                    selection.nodes.add(nodeId);
                }
            }

            if(oldActiveNode && selection.nodes.has(oldActiveNode)) {
                selection.activeNode = oldActiveNode;
            }

            selection = { ...selection }; // Trigger reactivity

            event.preventDefault();
        }}
    ></div>
    <div
        class="marquee"
        style="
            left: {Math.min(marquee.startX, marquee.endX)}px;
            top: {Math.min(marquee.startY, marquee.endY)}px;
            width: {Math.abs(marquee.endX - marquee.startX)}px;
            height: {Math.abs(marquee.endY - marquee.startY)}px;
        "
    ></div>
{/if}

<style>
.marquee {
    position: absolute;
    border: 1px dashed var(--text);
    border-radius: 2px;
    background-color: #6666661a;
    pointer-events: none;
}

.marquee-selection-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: all;
}
</style>