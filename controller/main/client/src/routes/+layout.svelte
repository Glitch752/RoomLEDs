<script lang="ts">
    import { onMount, type Snippet } from "svelte";
    import "../app.scss";
    import { initWebsocket } from "$lib/websocket";
    import { updateDocumentTheme } from "../../settings.svelte";

    const { children }: { children: Snippet } = $props();

    updateDocumentTheme();
    
    onMount(() => {
        initWebsocket();
    });
</script>

<svelte:head>
    <title>Lighting control interface</title>
</svelte:head>

{@render children()}