<script lang="ts">
    import type { Snippet } from "svelte";
    import NEContextMenu from "./NEContextMenu.svelte";
    import NEContextMenuItem from "./NEContextMenuItem.svelte";

    const {
        label,
        path,
        children
    }: {
        label: string,
        path?: string,
        children?: Snippet
    } = $props();
</script>

<NEContextMenuItem label={label}>
    <span class="submenu-indicator">▶</span>
</NEContextMenuItem>
<NEContextMenu>
    {@render children?.()}
</NEContextMenu>