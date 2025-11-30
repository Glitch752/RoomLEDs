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

let pixelValues: Uint8Array | null = null;
let litPixelValues: Uint8Array | null = null;

function getCanvasPosition(x: number, y: number) {
    return {
        x: (x - cameraPosition.x) * cameraZoom + canvas.width / 2,
        y: (y - cameraPosition.y) * cameraZoom + canvas.height / 2
    };
}

function draw() {
    if(!ctx) return;
    if(!lightPositions) return;
    
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    const { positions, xMax, xMin, yMax, yMin } = lightPositions;

    const bounds = canvas.getBoundingClientRect();

    if(!pixelValues || pixelValues.length !== positions.length * 3) {
        pixelValues = new Uint8Array(positions.length * 3);
    }
    if(!litPixelValues || litPixelValues.length !== positions.length * 3) {
        litPixelValues = new Uint8Array(positions.length * 3);
    }

    for(let i = 0; i < positions.length; i++) {
        const position = positions[i];
        const { x, y } = getCanvasPosition(position.x, position.y);
        const distance = Math.sqrt((mousePosition.x - bounds.left - x) ** 2 + (mousePosition.y - bounds.top - y) ** 2);
        
        // If the pixel has any value other than 0 in litPixelValues, use that color instead
        if(litPixelValues[i * 3] !== 0 || litPixelValues[i * 3 + 1] !== 0 || litPixelValues[i * 3 + 2] !== 0) {
            ctx.fillStyle = `rgba(${litPixelValues[i * 3]}, ${litPixelValues[i * 3 + 1]}, ${litPixelValues[i * 3 + 2]}, 1)`;
            
            pixelValues[i * 3] = litPixelValues[i * 3];
            pixelValues[i * 3 + 1] = litPixelValues[i * 3 + 1];
            pixelValues[i * 3 + 2] = litPixelValues[i * 3 + 2];

            // If hovered, draw the light index at the top of the screen
            if(distance < 20) {
                ctx.fillStyle = "white";
                ctx.font = "20px Arial";
                ctx.fillText(`Pixel index: ${i}`, 10, 20);
            }
        } else {
            if(distance > 20) {
                const brightness = Math.max(0, 100 - distance / 4);
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
        }

        ctx.beginPath();
        ctx.arc(x, y, 5, 0, 2 * Math.PI);
        ctx.fill();
    }

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

    // If within 20 pixels of any lights, toggle the nearest light
    if(litPixelValues && lightPositions) {
        const { positions } = lightPositions;

        const bounds = canvas.getBoundingClientRect();

        let closestDistance = Infinity;
        let closestIndex = -1;
        for(let i = 0; i < positions.length; i++) {
            const position = positions[i];
            const { x, y } = getCanvasPosition(position.x, position.y);
            const distance = Math.sqrt((mousePosition.x - bounds.left - x) ** 2 + (mousePosition.y - bounds.top - y) ** 2);

            if(distance < closestDistance) {
                closestDistance = distance;
                closestIndex = i;
            }
        }

        if(closestDistance < 20) {
            const index = closestIndex * 3;
            const value = litPixelValues[index] === 0 ? 255 : 0;

            litPixelValues[index] = value;
            litPixelValues[index + 1] = value;
            litPixelValues[index + 2] = value;
        }
    }
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

    background-color: var(--surface0);
}
</style>