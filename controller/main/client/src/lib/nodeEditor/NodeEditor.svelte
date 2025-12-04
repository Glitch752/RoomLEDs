<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import NEBackgroundCanvas from "./NEBackgroundCanvas.svelte";
    import NEEdge from "./edge/NEEdge.svelte";
    import NENode from "./node/NENode.svelte";
    import NESelection from "./NESelection.svelte";
    import NESelector from "./NESelector.svelte";
    import type { CameraState, MarqueeState } from "./NodeTypes";
    import CallbackContainer from "../../util/callbackContainer";
    import NodeEditorState from './NodeEditorState';
    import NEDraggingEdge from "./edge/NEDraggingEdge.svelte";

    let editorElement: HTMLDivElement;
    const nodeState = new NodeEditorState();

    onMount(() => {
        nodeState.editorElement = editorElement;

        // @ts-ignore
        window.ns = nodeState; // for debugging
        // @ts-ignore
        return () => { window.ns = undefined; };
    });

    const nodes = nodeState.nodes;
    const edges = nodeState.edges;
    const camera = nodeState.camera;

    let targetCamera: CameraState = {
        center: { x: 0, y: 0 },
        zoom: 1
    };

    /**
     * exponential decay function to smoothly interpolate between two values
     * while properly respecting delta time.
     * @param a
     * @param b
     * @param decay decay rate; reasonable values are around 1 to 10
     * @param dt
     */
    function expDecay(a: number, b: number, decay: number, dt: number) {
        return a + (b - a) * Math.exp(-decay * dt);
    }

    let animFrame: number | null = null;
    onDestroy(() => {
        if(animFrame !== null) cancelAnimationFrame(animFrame);
    });
    let lastTime = performance.now();
    let renderCallbacks = new CallbackContainer();

    function render(time = performance.now()) {
        const dt = (time - lastTime) / 1000;
        lastTime = time;
        if(
            Math.abs($camera.center.x - targetCamera.center.x) > 0.01 ||
            Math.abs($camera.center.y - targetCamera.center.y) > 0.01 ||
            Math.abs($camera.zoom - targetCamera.zoom) > 0.0005
        ) {
            camera.update(c => {
                return {
                    ...c,
                    center: {
                        x: expDecay(c.center.x, targetCamera.center.x, 10, dt),
                        y: expDecay(c.center.y, targetCamera.center.y, 10, dt)
                    },
                    zoom: expDecay(c.zoom, targetCamera.zoom, 10, dt)
                };
            });
        }
        
        renderCallbacks.invokeAll();

        animFrame = requestAnimationFrame(render);
    }
    render();

    let marquee: MarqueeState = $state({ active: false, startX: 0, startY: 0, endX: 0, endY: 0 });

    function handleWheel(event: WheelEvent) {
        if (event.ctrlKey) {
            // Zoom when holding control
            const zoomFactor = 0.1;
            targetCamera.zoom = Math.max(0.1, Math.min(10,
                targetCamera.zoom * (1 - event.deltaY * zoomFactor / 100)
             ))
         } else {
             // Pan camera
             // read current camera zoom from the store
             const currentZoom = $camera.zoom;
             targetCamera.center.x += event.deltaX / currentZoom / 2;
             targetCamera.center.y += event.deltaY / currentZoom / 2;
         }
         event.preventDefault();
     }
</script>

<!-- 
    TODO: Zones
    https://raw.githubusercontent.com/brian3kb/graham_scan_js/master/src/graham_scan.js
-->

<svelte:document onkeydown={(e) => {
    // If the mouse is over the editor, forward keydown events to the node editor state
    // this isn't a very robust way to do it, but it matches Blender's
    // behavior and works for now.
    if(editorElement && editorElement.matches(':hover')) {
        nodeState.onkeydown(e);
    }
}} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="editor"
    onwheel={handleWheel}
    style="--zoom: {$camera.zoom};"
    bind:this={editorElement}
>
    <NEBackgroundCanvas camera={$camera} {renderCallbacks} />
    <NESelector bind:marquee />
    <div class="canvas" style="transform: scale({$camera.zoom}) translate(50%, 50%) translate({-$camera.center.x}px, {-$camera.center.y}px);">
        <svg class="edges">
            {#each $edges as edge (edge.id)}
                <NEEdge {edge} {nodeState} />
            {/each}
            <NEDraggingEdge {nodeState} />
        </svg>
        <div class="nodes">
            {#each $nodes as [id, node] (id)}
                <NENode id={id} {nodeState} />
            {/each}
        </div>
    </div>
    <NESelection bind:marquee selection={nodeState.selection} />
</div>

<style>
.editor {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.canvas {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    transform-origin: center;
    pointer-events: none;

    * {
        pointer-events: all;
    }
}

.edges {
    pointer-events: none;
    overflow: visible;
}
</style>