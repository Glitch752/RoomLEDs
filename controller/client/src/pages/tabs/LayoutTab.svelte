<script lang="ts">
import { onDestroy, onMount } from "svelte";
import { lightPositions } from "../../websocket";

let content: HTMLDivElement;
let canvas: HTMLCanvasElement;
let ctx: CanvasRenderingContext2D | null;

function draw() {
    if(!ctx) return;
    if(!lightPositions) return;
    
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // ctx.fillStyle = "red";
    // ctx.fillRect(0, 0, canvas.width, canvas.height);
    const positions = lightPositions.positions;
    for(const position of positions) {
        ctx.beginPath();
        ctx.arc(position.x * canvas.width, position.y * canvas.height, 10, 0, 2 * Math.PI);
        ctx.fill();
    }
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

onMount(() => {
    ctx = canvas.getContext("2d");
    if(!ctx) return;

    resizeObserver.observe(content);

    draw();
});

onDestroy(() => {
    resizeObserver.disconnect();
});
</script>

<div class="content" bind:this={content}>
    <canvas bind:this={canvas}></canvas>
</div>

<style lang="scss">
.content {
    width: 100%;
    height: 100%;
}
canvas {
    background-color: #f0f0f0;
}
</style>