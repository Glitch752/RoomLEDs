<script lang="ts">
    import { onMount } from "svelte";
    import type { NodeID, SelectionState } from "../NodeTypes";
    import type NodeEditorState from "../NodeEditorState";
    import NENodeLine from "./NENodeLine.svelte";
    import NENodeDataValue from "./NENodeDataValue.svelte";

    let {
        id,
        nodeState
    }: {
        id: NodeID,
        nodeState: NodeEditorState
    } = $props();

    let nodeElement: HTMLDivElement;

    const camera = nodeState.camera;
    const selection = nodeState.selection;

    const node = $derived.by(() => nodeState.getNode(id));

    function updateSocketCache() {
        // TODO: this is a garbage way to do this, and I recognize it, but
        // I don't know a better one lol
        const inputSockets = nodeElement.querySelectorAll(`.line.input .socket`);
        const outputSockets = nodeElement.querySelectorAll(`.line.output .socket`);

        // get the canvas-space heights of the sockets based on their bounding boxes
        // relative to the node editor canvas
        const nodeBounds = nodeElement.getBoundingClientRect();
        node.update((n) => {
            n.inputPositionCache = Array.from(inputSockets).map(socket => {
                const socketBounds = socket.getBoundingClientRect();
                return (socketBounds.y + socketBounds.height / 2 - nodeBounds.y) / $camera.zoom
            });
            n.outputPositionCache = Array.from(outputSockets).map(socket => {
                const socketBounds = socket.getBoundingClientRect();
                return (socketBounds.y + socketBounds.height / 2 - nodeBounds.y) / $camera.zoom
            });
            return n;
        });
    }

    onMount(() => {
        updateSocketCache();
    });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
    class="node"
    class:active={$selection.activeNode === id}
    class:selected={$selection.nodes.has(id)}
    data-node-id={id}

    bind:this={nodeElement}
    style="left:{$node.x}px;top:{$node.y}px;width:{$node.width}px;"
>
    <div class="title" title={$node.variantInfo.description}>{$node.variantInfo.name}</div>

    <div class="lines">
        {#each Object.entries($node.variantInfo.dataValues) as [name, dataValue] (name)}
            <NENodeDataValue key={name} descriptor={dataValue} value={$node.dataValues[name]} onchange={(newValue) => {
                node.update(n => {
                    n.dataValues[name] = newValue;
                    return n;
                });
            }} />
        {/each}

        {#each $node.variantInfo.outputs as output, i}
            <NENodeLine isInput={false} {nodeState} node={$node} index={i} text={output.label} type={output.type} />
        {/each}
        
        {#each $node.variantInfo.inputs as input, i}
            <NENodeLine isInput={true} {nodeState} node={$node} index={i} text={input.label} type={input.type} />
        {/each}
    </div>
</div>

<style lang="scss">
.node {
    --border-radius: 4px;

    position: absolute;

    background: var(--surface0);
    border: 1px solid var(--surface1);
    box-shadow: 0 2px 5px rgba(0,0,0,0.5);

    color: var(--text);
    border-radius: var(--border-radius);
    cursor: grab;
    user-select: none;

    &.active {
        border-color: white;
    }
    &.selected:not(.active) {
        border-color: var(--peach);
        border-width: calc(max(1px, 1.5px / var(--zoom)));
    }
}
.node .title {
    text-align: left;
    padding: 1px 8px;
    background: var(--surface1);
    font-size: 0.9rem;

    border-radius: var(--border-radius) var(--border-radius) 0 0;
}

.lines {
    display: flex;
    flex-direction: column;
    padding: 0.25rem 0;
}
</style>
