<!-- Node.svelte -->
<script lang="ts">
    import { onMount } from "svelte";
    import type { CameraState, NodeData } from "./NodeTypes";

    const {
        node = $bindable(),
        camera
    }: {
        node: NodeData,
        camera: CameraState
    } = $props();

    let nodeElement: HTMLDivElement;

    function onDrag(event: MouseEvent) {
        node.x += event.movementX / camera.zoom;
        node.y += event.movementY / camera.zoom;
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
    bind:this={nodeElement}
    style="position:absolute; left:{node.x}px; top:{node.y}px; width:{node.width}px;"
    onmousedown={e => {
        window.addEventListener('mousemove', onDrag);
        window.addEventListener('mouseup', () => window.removeEventListener('mousemove', onDrag), { once: true });
        e.preventDefault();
    }}
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
    background: var(--surface0);
    border: 1px solid var(--surface1);
    box-shadow: 0 2px 5px rgba(0,0,0,0.5);

    color: var(--text);
    border-radius: 4px;
    cursor: grab;
    user-select: none;
}
.node .title {
    text-align: left;
    padding: 1px 8px;
    background: var(--surface1);
    font-size: 0.9rem;
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
