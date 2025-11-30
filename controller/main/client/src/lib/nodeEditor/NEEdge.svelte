<script lang="ts">
    import type { EdgeData, NodeData } from "./NodeTypes";

    const {
        nodes,
        edge
    }: {
        nodes: NodeData[],
        edge: EdgeData
    } = $props();

    const startNode = $derived.by(() => nodes.find(n => n.id === edge.from.nodeId));
    const endNode = $derived.by(() => nodes.find(n => n.id === edge.to.nodeId));

    const startNodePos = $derived.by(() => startNode ? ({
        x: startNode.x + startNode.width,
        y: startNode.y + (startNode.outputPositionCache?.[edge.from.outputIndex] ?? 0)
    }) : { x: 0, y: 0 });
    const endNodePos = $derived.by(() => endNode ? ({
        x: endNode.x,
        y: endNode.y + (endNode.inputPositionCache?.[edge.to.inputIndex] ?? 0)
    }) : { x: 0, y: 0 });

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