<script lang="ts">
    import { onDestroy } from "svelte";
    import NEBackgroundCanvas from "./NEBackgroundCanvas.svelte";
    import NEEdge from "./NEEdge.svelte";
    import NENode from "./NENode.svelte";
    import NESelection from "./NESelection.svelte";
    import NESelector from "./NESelector.svelte";
    import type { CameraState, EdgeData, MarqueeState, NodeData, SelectionState } from "./NodeTypes";
    import CallbackContainer from "../../util/callbackContainer";

    const nodes: NodeData[] = $state([
        { id: 'node1', x: 100, y: 100, width: 150, label: 'Input A', inputs: [], outputs: ['Value'], zIndex: 0 },
        { id: 'node2', x: 100, y: 300, width: 150, label: 'Input B', inputs: [], outputs: ['Value'], zIndex: 1 },
        { id: 'node3', x: 300, y: 200, width: 150, label: 'Add', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 2 },
        { id: 'node4', x: 500, y: 100, width: 150, label: 'Multiply', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 3 },
        { id: 'node5', x: 500, y: 300, width: 150, label: 'Subtract', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 4 },
        { id: 'node6', x: 700, y: 200, width: 150, label: 'Divide', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 5 },
        { id: 'node7', x: 900, y: 200, width: 150, label: 'Output', inputs: ['Value'], outputs: [], zIndex: 6 },
        { id: 'node8', x: 300, y: 400, width: 150, label: 'Constant C', inputs: [], outputs: ['Value'], zIndex: 7 },
        { id: 'node9', x: 500, y: 500, width: 150, label: 'Power', inputs: ['Base', 'Exponent'], outputs: ['Result'], zIndex: 8 },
        { id: 'node10', x: 700, y: 400, width: 150, label: 'Modulo', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 9 },
        { id: 'node11', x: 900, y: 400, width: 150, label: 'Output 2', inputs: ['Value'], outputs: [], zIndex: 10 },
        { id: 'node12', x: 100, y: 500, width: 150, label: 'Input D', inputs: [], outputs: ['Value'], zIndex: 11 },
        { id: 'node13', x: 300, y: 600, width: 150, label: 'Logarithm', inputs: ['Value'], outputs: ['Result'], zIndex: 12 },
        { id: 'node14', x: 500, y: 700, width: 150, label: 'Square Root', inputs: ['Value'], outputs: ['Result'], zIndex: 13 },
        { id: 'node15', x: 700, y: 600, width: 150, label: 'Sin', inputs: ['Angle'], outputs: ['Result'], zIndex: 14 },
        { id: 'node16', x: 900, y: 600, width: 150, label: 'Cos', inputs: ['Angle'], outputs: ['Result'], zIndex: 15 },
        { id: 'node17', x: 1100, y: 200, width: 150, label: 'Final Output', inputs: ['Value'], outputs: [], zIndex: 16 },
        { id: 'node18', x: 1100, y: 400, width: 150, label: 'Final Output 2', inputs: ['Value'], outputs: [], zIndex: 17 },
        { id: 'node19', x: 1100, y: 600, width: 150, label: 'Final Output 3', inputs: ['Value'], outputs: [], zIndex: 18 }
    ]);

    const edges: EdgeData[] = $state([
        { id: 'edge1', from: { nodeId: 'node1', outputIndex: 0 }, to: { nodeId: 'node3', inputIndex: 0 } },
        { id: 'edge2', from: { nodeId: 'node2', outputIndex: 0 }, to: { nodeId: 'node3', inputIndex: 1 } },
        { id: 'edge3', from: { nodeId: 'node3', outputIndex: 0 }, to: { nodeId: 'node4', inputIndex: 0 } },
        { id: 'edge4', from: { nodeId: 'node8', outputIndex: 0 }, to: { nodeId: 'node4', inputIndex: 1 } },
        { id: 'edge5', from: { nodeId: 'node4', outputIndex: 0 }, to: { nodeId: 'node6', inputIndex: 0 } },
        { id: 'edge6', from: { nodeId: 'node5', outputIndex: 0 }, to: { nodeId: 'node6', inputIndex: 1 } },
        { id: 'edge7', from: { nodeId: 'node6', outputIndex: 0 }, to: { nodeId: 'node7', inputIndex: 0 } },
        { id: 'edge8', from: { nodeId: 'node9', outputIndex: 0 }, to: { nodeId: 'node10', inputIndex: 0 } },
        { id: 'edge9', from: { nodeId: 'node10', outputIndex: 0 }, to: { nodeId: 'node11', inputIndex: 0 } },
        { id: 'edge10', from: { nodeId: 'node12', outputIndex: 0 }, to: { nodeId: 'node13', inputIndex: 0 } },
        { id: 'edge11', from: { nodeId: 'node13', outputIndex: 0 }, to: { nodeId: 'node14', inputIndex: 0 } },
        { id: 'edge12', from: { nodeId: 'node14', outputIndex: 0 }, to: { nodeId: 'node15', inputIndex: 0 } },
        { id: 'edge13', from: { nodeId: 'node15', outputIndex: 0 }, to: { nodeId: 'node16', inputIndex: 0 } },
        { id: 'edge14', from: { nodeId: 'node16', outputIndex: 0 }, to: { nodeId: 'node17', inputIndex: 0 } },
        { id: 'edgeidk', from: { nodeId: 'node8', outputIndex: 0 }, to: { nodeId: 'node9', inputIndex: 0 } },
        { id: 'edgeidk2', from: { nodeId: 'node8', outputIndex: 0 }, to: { nodeId: 'node5', inputIndex: 0 } },
        { id: 'edgeidk3', from: { nodeId: 'node8', outputIndex: 0 }, to: { nodeId: 'node5', inputIndex: 1 } }
    ]);

    const camera: CameraState = $state({
        center: { x: 0, y: 0 },
        zoom: 1
    });

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
            Math.abs(camera.center.x - targetCamera.center.x) > 0.01 ||
            Math.abs(camera.center.y - targetCamera.center.y) > 0.01 ||
            Math.abs(camera.zoom - targetCamera.zoom) > 0.0005
        ) {
            camera.center.x = expDecay(camera.center.x, targetCamera.center.x, 10, dt);
            camera.center.y = expDecay(camera.center.y, targetCamera.center.y, 10, dt);
            camera.zoom = expDecay(camera.zoom, targetCamera.zoom, 10, dt);
        }
        
        renderCallbacks.invokeAll();

        animFrame = requestAnimationFrame(render);
    }
    render();

    let selection: SelectionState = $state({
        nodes: new Set<string>(),
        activeNode: null
    });
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
            targetCamera.center.x -= event.deltaX / camera.zoom / 2;
            targetCamera.center.y -= event.deltaY / camera.zoom / 2;
        }
        event.preventDefault();
    }
</script>

<!-- 
    TODO: Zones
    https://raw.githubusercontent.com/brian3kb/graham_scan_js/master/src/graham_scan.js
-->

<div class="editor" onwheel={handleWheel} style="--zoom: {camera.zoom};">
    <NEBackgroundCanvas {camera} {renderCallbacks} />
    <NESelector bind:marquee />
    <div class="canvas" style="transform: scale({camera.zoom}) translate({camera.center.x}px, {camera.center.y}px);">
        <svg class="edges">
            {#each edges as edge (edge.id)}
                <NEEdge {edge} {nodes} />
            {/each}
        </svg>
        <div class="nodes">
            {#each nodes as node, i (node.id)}
                <NENode bind:node={nodes[i]} {camera} bind:selection {nodes} />
            {/each}
        </div>
    </div>
    <NESelection {nodes} bind:marquee bind:selection />
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