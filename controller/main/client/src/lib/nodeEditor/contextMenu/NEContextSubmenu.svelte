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

    let menu: NEContextMenu | null = $state(null);
</script>

<NEContextMenuItem
    label={label}
    onfocus={(e) => menu?.openAtSubmenu(e.currentTarget as HTMLElement)}
    onunfocus={() => menu?.close()}
>
    <span class="submenu-indicator">▶</span>
</NEContextMenuItem>
<NEContextMenu bind:this={menu}>
    {@render children?.()}
</NEContextMenu>

<style lang="scss">
.submenu-indicator {
    font-size: 0.6em;
    margin-top: 0.3em;
}
</style>