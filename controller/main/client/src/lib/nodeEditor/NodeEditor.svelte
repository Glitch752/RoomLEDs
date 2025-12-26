<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import NEBackgroundCanvas from "./NEBackgroundCanvas.svelte";
    import NEEdge from "./edge/NEEdge.svelte";
    import NENode from "./node/NENode.svelte";
    import NESelection from "./NESelection.svelte";
    import NESelector from "./NESelector.svelte";
    import type { CameraState, MarqueeState } from "./NodeTypes";
    import CallbackContainer from "../../util/callbackContainer";
    import NEDocumentState from './NEDocumentState';
    import NEDraggingEdge from "./edge/NEDraggingEdge.svelte";
    import { expDecay } from "../../util/timing";

    let editorElement: HTMLDivElement;
    const nodeState = new NEDocumentState();

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

    // TODO: A proper settings interface
    const settings = {
        zoomWithWheel: true
    };

    function handleWheel(event: WheelEvent) {
        let shouldPan = false, panX = false;
        if(settings.zoomWithWheel) {
            shouldPan = event.ctrlKey || event.shiftKey;
            panX = event.ctrlKey;
        } else {
            shouldPan = !event.ctrlKey;
            panX = event.shiftKey;
        }
        
        if(shouldPan) {
            const deltaX = panX ? event.deltaY : event.deltaX;
            const deltaY = panX ? 0 : event.deltaY;

            const currentZoom = $camera.zoom;
            targetCamera.center.x += deltaX / currentZoom / 2;
            targetCamera.center.y += deltaY / currentZoom / 2;
        } else {
            const zoomFactor = 0.1;
            targetCamera.zoom = Math.max(0.1, Math.min(10,
                targetCamera.zoom * (1 - event.deltaY * zoomFactor / 100)
            ));
        }
        event.preventDefault();
    }

    onMount(() => {
        function beginMiddlePan(e: MouseEvent) {
            const startX = e.clientX, startY = e.clientY;
            const startCameraX = $camera.center.x, startCameraY = $camera.center.y;
            
            nodeState.handleMouseMoveUntilMouseUp((moveEvent) => {
                const deltaX = moveEvent.clientX - startX;
                const deltaY = moveEvent.clientY - startY;

                const currentZoom = $camera.zoom;
                targetCamera.center.x = startCameraX - deltaX / currentZoom;
                targetCamera.center.y = startCameraY - deltaY / currentZoom;
            });
        }

        const unsubscribe = nodeState.handleMouseDown((e: MouseEvent) => {
            if(e.button === 1) { // middle mouse button
                beginMiddlePan.call(nodeState, e);
                e.preventDefault();
                e.stopPropagation();
            }
        });
        return unsubscribe;
    })
</script>

<!-- 
    TODO: Zones
    https://raw.githubusercontent.com/brian3kb/graham_scan_js/master/src/graham_scan.js
-->

<svelte:document
    onkeydown={(e) => {
        // If the mouse is over the editor, forward keydown events to the node editor state
        // this isn't a very robust way to do it, but it matches Blender's
        // behavior and works for now.
        if(editorElement && editorElement.matches(':hover')) {
            nodeState.onkeydown(e);
        }
    }}
    onmousemove={(e) => nodeState.onmousemove(e)}
    onmousedown={(e) => nodeState.onmousedown(e)}
    onmouseup={(e) => nodeState.onmouseup(e)}
/>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="editor"
    onwheel={handleWheel}
    style="--zoom: {$camera.zoom};"
    bind:this={editorElement}
>
    <NEBackgroundCanvas camera={$camera} {renderCallbacks} />
    <NESelector bind:marquee editMode={nodeState.editMode} />
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