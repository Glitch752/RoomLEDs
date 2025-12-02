<script lang="ts">
    import NEPositionedEdge from "./NEPositionedEdge.svelte";
    import type NodeEditorState from "../NodeEditorState";
    import type { EdgeData } from "../NodeTypes";
    import { nodeDataTypeInfo } from "../NodeVariants";

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

    const type = $derived($startNode.variantInfo.outputs[edge.from.outputIndex]?.type);
    const typeInfo = $derived(nodeDataTypeInfo[type]);
</script>

<NEPositionedEdge
    start={startNodePos}
    end={endNodePos}
    primaryColor={typeInfo.primaryColor}
    alphaBackgroundColor={typeInfo.alphaBackgroundColor}
/>