<script lang="ts">
    import type { Snippet } from "svelte";

    const {
        label,
        onselect,
        children
    }: {
        label: string,
        onselect?: () => void,
        children?: Snippet
    } = $props();
</script>

<button class="context-menu-item" onclick={onselect}>
    <span class="label">{label}</span>
    {@render children?.()}
</button>

<style lang="scss">
.context-menu-item {
    width: 100%;
    padding: 0.25rem 0.5rem;
    background: none;
    border: none;
    text-align: left;
    cursor: pointer;
    font-size: inherit;
    font-weight: inherit;

    display: flex;
    flex-direction: row;

    &:hover {
        background-color: var(--surface1);
    }

    .label {
        flex-grow: 1;
    }
}
</style>