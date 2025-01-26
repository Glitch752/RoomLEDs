<script lang="ts">
import { onDestroy, onMount } from "svelte";
import { lightPositions, sendLightData } from "../../websocket";

let content: HTMLDivElement;
let canvas: HTMLCanvasElement;
let ctx: CanvasRenderingContext2D | null;

let cameraZoom = 1500;
let cameraPosition = { x: 0, y: 0 };

let mousePosition = { x: 0, y: 0 };
let mouseDown = false;

function getCanvasPosition(x: number, y: number) {
    return {
        x: (x - cameraPosition.x) * cameraZoom + canvas.width / 2,
        y: (y - cameraPosition.y) * cameraZoom + canvas.height / 2
    };
}

function draw() {
    if(!ctx) return;
    if(!lightPositions) return;
    
    ctx.fillStyle = "#1f1f1f";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    const { positions, xMax, xMin, yMax, yMin } = lightPositions;

    const bounds = canvas.getBoundingClientRect();

    let pixelValues = new Uint8Array(positions.length * 3);

    for(let i = 0; i < positions.length; i++) {
        const position = positions[i];

        const { x, y } = getCanvasPosition(position.x, position.y);
        const distance = Math.sqrt((mousePosition.x - bounds.left - x) ** 2 + (mousePosition.y - bounds.top - y) ** 2);
        
        if(distance > 20) {
            const brightness = Math.max(0, 255 - distance / 2);
            ctx.fillStyle = `rgba(${brightness}, ${brightness}, ${brightness}, 1)`;

            pixelValues[i * 3] = brightness;
            pixelValues[i * 3 + 1] = brightness;
            pixelValues[i * 3 + 2] = brightness;
        } else {
            ctx.fillStyle = "yellow";

            pixelValues[i * 3] = 255;
            pixelValues[i * 3 + 1] = 255;
            pixelValues[i * 3 + 2] = 0;
        }

        ctx.beginPath();
        ctx.arc(x, y, 5, 0, 2 * Math.PI);
        ctx.fill();
    }

    console.log("Sending light data", pixelValues);

    sendLightData(pixelValues);
}

function handleWheel(event: WheelEvent) {
    const bounds = canvas.getBoundingClientRect();
    const canvasX = event.clientX - bounds.left;
    const canvasY = event.clientY - bounds.top;

    // Zoom into the cursor position
    const zoomFactor = Math.pow(1.1, event.deltaY * -0.01);
    const oldZoom = cameraZoom;
    
    cameraZoom *= zoomFactor;

    const zoomDiff = cameraZoom - oldZoom;
    cameraPosition.x += (canvasX - canvas.width / 2) / oldZoom * zoomDiff / cameraZoom;
    cameraPosition.y += (canvasY - canvas.height / 2) / oldZoom * zoomDiff / cameraZoom;

    draw();
}

function handleMouseDown(event: MouseEvent) {
    mouseDown = true;
    mousePosition = { x: event.clientX, y: event.clientY };
}

function handleMouseMove(event: MouseEvent) {
    const dx = event.clientX - mousePosition.x;
    const dy = event.clientY - mousePosition.y;

    mousePosition = { x: event.clientX, y: event.clientY };

    if(!mouseDown) return;

    cameraPosition.x -= dx / cameraZoom;
    cameraPosition.y -= dy / cameraZoom;

    draw();
}

function handleMouseUp(event: MouseEvent) {
    mouseDown = false;
}

const resizeObserver = new ResizeObserver((event) => {
    if(!canvas) return;
    if(!event[0]) return;

    const { width, height } = event[0].contentRect;

    canvas.width = width;
    canvas.height = height;

    console.log("Resized canvas to", width, height);

    draw();
});

let running = true;
onMount(() => {
    ctx = canvas.getContext("2d");
    if(!ctx) return;

    resizeObserver.observe(content);

    canvas.addEventListener("wheel", handleWheel);
    canvas.addEventListener("mousedown", handleMouseDown);
    canvas.addEventListener("mousemove", handleMouseMove);
    canvas.addEventListener("mouseup", handleMouseUp);

    const drawLoop = () => {
        if(running) {
            draw();
            requestAnimationFrame(drawLoop);
        }
    };

    drawLoop();
});

onDestroy(() => {
    resizeObserver.disconnect();

    canvas.removeEventListener("wheel", handleWheel);
    canvas.removeEventListener("mousedown", handleMouseDown);
    canvas.removeEventListener("mousemove", handleMouseMove);
    canvas.removeEventListener("mouseup", handleMouseUp);

    running = false;
});
</script>

<div class="content" bind:this={content}>
    <canvas bind:this={canvas} width="800" height="600"></canvas>
</div>

<style lang="scss">
.content {
    width: 100%;
    height: 100%;
}
canvas {
    width: 100%;
    height: 100%;

    background-color: #f0f0f0;
}
</style>