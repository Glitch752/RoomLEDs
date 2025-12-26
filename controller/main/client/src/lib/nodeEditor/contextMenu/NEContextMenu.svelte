<script lang="ts">
  import type { Snippet } from "svelte";

let x: number = $state(0);
let y: number = $state(0);
let isOpen: boolean = $state(false);

const {
    children
}: {
    children?: Snippet
} = $props();

let mouseX = 0, mouseY = 0;
function onmousemove(event: MouseEvent) {
    mouseX = event.clientX;
    mouseY = event.clientY;

    if(isOpen && !(event.target as HTMLElement).closest('.context-menu-wrapper')) {
        close();
    }
}

export function openAtMouse(path?: string[]) {
    x = mouseX - 50;
    y = mouseY - 10;
    isOpen = true;
}

export function openAtSubmenu(submenuElement: HTMLElement) {
    const rect = submenuElement.getBoundingClientRect();
    x = rect.right;
    y = rect.top;
    isOpen = true;
}

export function close() {
    isOpen = false;
}
</script>

<svelte:document {onmousemove} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
{#if isOpen}
    <div class="context-menu-wrapper" style="--y: {y}px; --x: {x}px;" onmousedown={close}>
        <div class="context-menu" onmousedown={(e) => e.stopPropagation()}>
            {@render children?.()}
        </div>
    </div>
{/if}

<style lang="scss">
.context-menu-wrapper {
    --hover-margin: 3rem;
    padding: var(--hover-margin);

    top: calc(var(--y) - var(--hover-margin));
    left: calc(var(--x) - var(--hover-margin));
    
    position: fixed;

    display: block;
}
.context-menu {
    background-color: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 4px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
    z-index: 1000;
    min-width: 150px;
    font-size: 14px;
    color: var(--text);
    
    display: flex;
    flex-direction: column;
}
</style>