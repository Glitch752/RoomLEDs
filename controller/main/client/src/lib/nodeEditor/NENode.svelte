<!-- Node.svelte -->
<script lang="ts">
    import { onMount } from "svelte";
    import type { CameraState, NodeData, SelectionState } from "./NodeTypes";

    let {
        nodes,
        node = $bindable(),
        camera,
        selection = $bindable()
    }: {
        nodes: NodeData[],
        node: NodeData,
        camera: CameraState,
        selection: SelectionState
    } = $props();

    let nodeElement: HTMLDivElement;

    let didMouseMove = false;
    function onDrag(event: MouseEvent) {
        // Drag every selected node
        for(const nodeId of selection.nodes) {
            const n = nodes.find(n => n.id === nodeId);
            if(!n) continue;

            n.x += event.movementX / camera.zoom;
            n.y += event.movementY / camera.zoom;
        }

        didMouseMove = true;
    }

    function handleMultiSelectMousedown(event: MouseEvent) {
        // if this node isn't active, make it the active selection
        // and ensure it's part of the selection set
        // otherwise, remove it from the selection entirely
        if(selection.activeNode !== node.id) {
            selection.nodes.add(node.id);
            selection.activeNode = node.id;
        } else {
            selection.nodes.delete(node.id);
            if(selection.activeNode === node.id) {
                selection.activeNode = null;
            }
        }
        selection = { ...selection }; // Trigger reactivity
    }

    function onmousedown(event: MouseEvent) {
        event.preventDefault();

        if(event.shiftKey) {
            handleMultiSelectMousedown(event);
            return;
        }

        window.addEventListener('mousemove', onDrag);
        window.addEventListener('mouseup', () => {
            window.removeEventListener('mousemove', onDrag);

            if(!didMouseMove) {
                // If the mouse didn't move, this was a click, so set ourself
                // as the active node
                selection.nodes.clear();
                selection.nodes.add(node.id);
                selection.activeNode = node.id;
                selection = { ...selection }; // Trigger reactivity
            }
        }, { once: true });

        didMouseMove = false;

        // If we're not part of the selection, set ourself as the sole active node
        if(!selection.nodes.has(node.id)) {
            selection.nodes.clear();
            selection.nodes.add(node.id);
            selection.activeNode = node.id;
            selection = { ...selection }; // Trigger reactivity
        }
    }

    function updateSocketCache() {
        // TODO: this is a garbage way to do this, and I recognize it, but
        // I don't know a better one lol
        const inputSockets = nodeElement.querySelectorAll(`.line.input .socket`);
        const outputSockets = nodeElement.querySelectorAll(`.line.output .socket`);

        // get the canvas-space heights of the sockets based on their bounding boxes
        // relative to the node editor canvas
        const nodeBounds = nodeElement.getBoundingClientRect();
        node.inputPositionCache = Array.from(inputSockets).map(socket => {
            const socketBounds = socket.getBoundingClientRect();
            return (socketBounds.y + socketBounds.height / 2 - nodeBounds.y) / camera.zoom
        });
        node.outputPositionCache = Array.from(outputSockets).map(socket => {
            const socketBounds = socket.getBoundingClientRect();
            return (socketBounds.y + socketBounds.height / 2 - nodeBounds.y) / camera.zoom
        });
    }

    onMount(() => {
        updateSocketCache();
    });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
    class="node"
    class:active={selection.activeNode === node.id}
    class:selected={selection.nodes.has(node.id)}
    data-node-id={node.id}

    bind:this={nodeElement}
    style="left:{node.x}px;top:{node.y}px;width:{node.width}px;"

    {onmousedown}
>
    <div class="title">{node.label}</div>

    <div class="lines">
        {#each node.outputs as output, i}
            <div class="line output">
                <span class="content">{output}</span>
                <div class="socket"></div>
            </div>
        {/each}
        
        {#each node.inputs as input, i}
            <div class="line input">
                <div class="socket"></div>
                <span class="content">{input}</span>
            </div>
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
.line {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0;
    gap: 0.7rem;

    .content {
        font-size: 0.85rem;
        flex: 1;
    }
    &.output .content {
        text-align: right;
    }
    &.input .content {
        text-align: left;
    }
    
    .socket {
        width: 6px;
        height: 10px;
        margin: 0 -3px;
        background: var(--subtext0);
        border: 1px solid var(--subtext1);
        border-radius: 3px;
    }
    &.input .socket {
        order: 0;
    }
}
</style>
