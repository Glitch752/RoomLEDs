<script lang="ts">
    import { onMount } from "svelte";
    import { lightData, lightPositions } from "../websocket";

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D | null;

    let blurcanvas: HTMLCanvasElement;
    let blurctx: CanvasRenderingContext2D | null;

    function getLightPosition(index: number): { x: number, y: number } {
        if(!lightPositions) return { x: 0, y: 0 };

        let { positions, xMin, xMax, yMin, yMax } = lightPositions;
        
        const MARGIN = 0.02;
        xMin -= MARGIN;
        xMax += MARGIN;
        yMin -= MARGIN;
        yMax += MARGIN;
        
        const { x, y } = positions[index % positions.length];

        let scaleFactor = Math.min(canvas.width / (xMax - xMin), canvas.height / (yMax - yMin));
        if(scaleFactor === Infinity) scaleFactor = 1;

        return {
            x: (x - xMin) * scaleFactor,
            y: (y - yMin) * scaleFactor
        };
    }

    function draw(ctx: CanvasRenderingContext2D) {
        if(lightData && canvas) {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            for(let i = 0; i < lightData.length; i += 3) {
                const size = 3;

                const { x, y } = getLightPosition(i / 3);

                const r = lightData[i + 0];
                const g = lightData[i + 1];
                const b = lightData[i + 2];

                ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;

                ctx.beginPath();
                ctx.arc(x, y, size, 0, Math.PI * 2);
                ctx.fill();
            }
        }

        // Copy the canvas to the blur canvas
        if(blurctx) {
            blurctx.clearRect(0, 0, blurcanvas.width, blurcanvas.height);
            blurctx.drawImage(canvas, 0, 0);
        }

        requestAnimationFrame(() => draw(ctx));
    }
    
    onMount(() => {
        ctx = canvas.getContext("2d");
        blurctx = blurcanvas.getContext("2d");
        if(!ctx) throw new Error("Could not get 2d context");
        if(!blurctx) throw new Error("Could not get 2d context");

        draw(ctx);
    });
</script>

<div class="container">
    <canvas width="800" height="800" class="blurcanvas" bind:this={blurcanvas}></canvas>
    <canvas width="800" height="800" bind:this={canvas}></canvas>
</div>

<style>
    .container {
        position: relative;
    }
    .blurcanvas {
        position: absolute;
        top: 0;
        left: 0;
        pointer-events: none;
        filter: saturate(1.5) blur(6px);
    }
</style>