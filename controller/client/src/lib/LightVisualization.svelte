<script lang="ts">
    import { onMount } from "svelte";
    import type { LightPosition } from "@bindings/LightPosition";

    export type LightPositionData = {
        positions: LightPosition[],
        xMin: number,
        xMax: number,
        yMin: number,
        yMax: number
    };

    const { lightData, lightPositions }: { lightData?: Uint8Array, lightPositions?: LightPositionData } = $props();

    // $state(0);
    
    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D | null;

    function getLightPosition(index: number): { x: number, y: number } {
        if(!lightPositions) return { x: 0, y: 0 };

        const { positions, xMin, xMax, yMin, yMax } = lightPositions;
        const { x, y } = positions[index % positions.length];

        let scaleFactor = Math.min(canvas.width / (xMax - xMin), canvas.height / (yMax - yMin));
        if(scaleFactor === Infinity) scaleFactor = 1;

        return {
            x: (x - xMin) * scaleFactor,
            y: (y - yMin) * scaleFactor
        };
    }
    
    onMount(() => {
        ctx = canvas.getContext("2d");
        if(!ctx) throw new Error("Could not get 2d context");

        if(!lightData) return;

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for(let i = 0; i < lightData.length; i += 3) {
            const size = 2;

            const { x, y } = getLightPosition(i / 3);

            const r = lightData[i + 0];
            const g = lightData[i + 1];
            const b = lightData[i + 2];

            ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;

            ctx.beginPath();
            ctx.arc(x, y, size, 0, Math.PI * 2);
            ctx.fill();
        }
    });
</script>

<canvas width="800" height="600" bind:this={canvas}></canvas>

<style>
    canvas {
        border: 1px solid black;
    }
</style>