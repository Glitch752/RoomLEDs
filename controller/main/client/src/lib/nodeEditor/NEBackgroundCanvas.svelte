<script lang="ts">
    import { onMount } from 'svelte';
    import type { CameraState } from './NodeTypes';
    import type CallbackContainer from '../../util/callbackContainer';

    const {
        camera,
        renderCallbacks
    }: {
        camera: CameraState,
        renderCallbacks: CallbackContainer
    } = $props();

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D;

    function getCSSColor(varName: string, fallback: string): string {
        return getComputedStyle(document.body).getPropertyValue(varName) || fallback;
    }

    function resizeCanvas() {
        canvas.width = (canvas.parentElement?.clientWidth ?? 800) * window.devicePixelRatio;
        canvas.height = (canvas.parentElement?.clientHeight ?? 600) * window.devicePixelRatio;

        draw();
    }

    function drawGrid(gridSize: number) {
        ctx.strokeStyle = getCSSColor('--surface1', '#000000');
        ctx.lineWidth = 1;

        const pixelRatioZoom = window.devicePixelRatio * camera.zoom;

        // Draw a grid of the given size on the canvas, respecting
        // the camera position and zoom
        const zoomedGridSize = gridSize * pixelRatioZoom;

        // Center of the canvas in screen coordinates
        const canvasCenterX = canvas.width / 2;
        const canvasCenterY = canvas.height / 2;

        // Offset so that camera.center is at the center of the canvas
        const offsetX = ((canvasCenterX - camera.center.x * pixelRatioZoom) % zoomedGridSize + zoomedGridSize) % zoomedGridSize;
        const offsetY = ((canvasCenterY - camera.center.y * pixelRatioZoom) % zoomedGridSize + zoomedGridSize) % zoomedGridSize;
        
        for(let x = offsetX; x < canvas.width; x += zoomedGridSize) {
            ctx.beginPath();
            ctx.moveTo(x, 0);
            ctx.lineTo(x, canvas.height);
            ctx.stroke();
        }
        for(let y = offsetY; y < canvas.height; y += zoomedGridSize) {
            ctx.beginPath();
            ctx.moveTo(0, y);
            ctx.lineTo(canvas.width, y);
            ctx.stroke();
        }
    }

    function draw() {
        if(!ctx) return;
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        const baseGridSize = 100;

        // Draw the nearest two powers of two based on the camera zoom
        // and smoothly fade between them

        const zoomLevel = Math.log2(camera.zoom);
        const lowerGridSize = baseGridSize / Math.pow(2, Math.floor(zoomLevel));
        const upperGridSize = baseGridSize / Math.pow(2, Math.ceil(zoomLevel));
        const upperWeight = zoomLevel - Math.floor(zoomLevel);
        const lowerWeight = 1 - upperWeight;

        ctx.globalAlpha = lowerWeight * 0.5;
        drawGrid(lowerGridSize);

        ctx.globalAlpha = upperWeight * 0.5;
        drawGrid(upperGridSize);
    }

    onMount(() => {
        ctx = canvas.getContext('2d')!;
        resizeCanvas();

        const resizeObserver = new ResizeObserver(() => resizeCanvas());
        resizeObserver.observe(canvas.parentElement!);

        renderCallbacks.addCallback(draw);

        return () => {
            resizeObserver.disconnect();
            renderCallbacks.removeCallback(draw);
        };
    });
</script>

<canvas bind:this={canvas}></canvas>

<style>
canvas {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: var(--background);
    z-index: -1;
}
</style>
