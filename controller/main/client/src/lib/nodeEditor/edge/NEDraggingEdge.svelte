<!-- TODO: Draw on a top layer with endpoints at snapped connections -->
<!-- TODO: Highlight attached ends to make it more obvious -->

<script lang="ts">
    import type NodeEditorState from "../NodeEditorState";
    import NEPositionedEdge from "./NEPositionedEdge.svelte";

    const {
        nodeState
    }: {
        nodeState: NodeEditorState
    } = $props();

    const edge = nodeState.draggingEdge;

    const attachedNode = $derived.by(() => {
        if(!$edge) return null;

        if($edge.type === "to") {
            return nodeState.getNode($edge.toNodeId);
        } else {
            return nodeState.getNode($edge.fromNodeId);
        }
    });
    const otherEndPosition = $derived.by(() => {
        if(!$edge) return { x: 0, y: 0 };

        if($edge.type === "to") {
            const from = $edge.from;
            if(from.type === "point") return from;
            
            // this technically may not be fully reactive..? but like whatever
            const fromNode = nodeState.getNodeValue(from.nodeId);
            if(!fromNode) return { x: 0, y: 0 };
            return {
                x: fromNode.x + fromNode.width,
                y: fromNode.y + (fromNode.outputPositionCache?.[from.outputIndex] ?? 0)
            };
        } else {
            const to = $edge.to;
            if(to.type === "point") return to;
            
            // this technically may not be fully reactive..? but like whatever
            const toNode = nodeState.getNodeValue(to.nodeId);
            if(!toNode) return { x: 0, y: 0 };
            return {
                x: toNode.x,
                y: toNode.y + (toNode.inputPositionCache?.[to.inputIndex] ?? 0)
            };
        }
    });
</script>

{#if $edge && $attachedNode}
    {#if $edge.type === "to"}
        <NEPositionedEdge
            start={otherEndPosition}
            end={{
                x: $attachedNode.x,
                y: $attachedNode.y + ($attachedNode.inputPositionCache?.[$edge.toInputIndex] ?? 0)
            }}
        ></NEPositionedEdge>
    {:else}
        <NEPositionedEdge
            start={{
                x: $attachedNode.x + $attachedNode.width,
                y: $attachedNode.y + ($attachedNode.outputPositionCache?.[$edge.fromOutputIndex] ?? 0)
            }}
            end={otherEndPosition}
        ></NEPositionedEdge>
    {/if}
{/if}