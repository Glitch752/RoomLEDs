<script lang="ts">
  import { onMount } from "svelte";

    // $state(0);
    
    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D | null;

    onMount(() => {
        ctx = canvas.getContext("2d");
        if(!ctx) {
            throw new Error("Could not get 2d context");
        }
        
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
    });
</script>

<canvas width="800" height="600" bind:this={canvas}></canvas>

<style>
    canvas {
        border: 1px solid black;
    }
</style>