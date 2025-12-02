<script lang="ts">
    const {
        start,
        end,
        primaryColor,
        alphaBackgroundColor
    }: {
        start: { x: number; y: number };
        end: { x: number; y: number };
        primaryColor: string;
        alphaBackgroundColor: string;
    } = $props();

    const bezierHandleDist = $derived(
        end.x < start.x ? // the end is to the left of the start, so scale the handle distance by their difference
            Math.abs(end.x - start.x) / 2
            : Math.min((end.x - start.x) / 3 + 100, (end.x - start.x) / 2)
    );

    const path = $derived(`M ${start.x} ${start.y} 
       C ${start.x + bezierHandleDist} ${start.y}, 
         ${end.x - bezierHandleDist} ${end.y}, 
         ${end.x} ${end.y}`);
</script>

<!-- Background outline -->
<path class="edge" d={path} stroke={alphaBackgroundColor} fill="none" stroke-width="4"/>
<path class="edge" d={path} stroke={primaryColor} fill="none" stroke-width="2"/>