<script lang="ts">
    import type NodeEditorState from "./NodeEditorState";
    import type { EdgeData } from "./NodeTypes";

    const {
        nodeState,
        edge
    }: {
        nodeState: NodeEditorState,
        edge: EdgeData
    } = $props();

    const startNode = $derived.by(() => nodeState.getNode(edge.from.nodeId));
    const endNode = $derived.by(() => nodeState.getNode(edge.to.nodeId));

    const startNodePos = $derived.by(() => ({
        x: $startNode.x + $startNode.width,
        y: $startNode.y + ($startNode.outputPositionCache?.[edge.from.outputIndex] ?? 0)
    }));
    const endNodePos = $derived.by(() => ({
        x: $endNode.x,
        y: $endNode.y + ($endNode.inputPositionCache?.[edge.to.inputIndex] ?? 0)
    }));

    const bezierHandleDist = $derived(
        endNodePos.x < startNodePos.x
            ? // the end is to the left of the start, so scale the handle distance by their difference
                Math.abs(endNodePos.x - startNodePos.x) / 2
            : Math.min(50, (endNodePos.x - startNodePos.x) / 2)
    );

    const path = $derived(`M ${startNodePos.x} ${startNodePos.y} 
       C ${startNodePos.x + bezierHandleDist} ${startNodePos.y}, 
         ${endNodePos.x - bezierHandleDist} ${endNodePos.y}, 
         ${endNodePos.x} ${endNodePos.y}`);
</script>

<!-- Background outline -->
<path class="edge" d={path} stroke="#ff000055" fill="none" stroke-width="4"/>
<path class="edge" d={path} stroke="#ff6666" fill="none" stroke-width="2"/>