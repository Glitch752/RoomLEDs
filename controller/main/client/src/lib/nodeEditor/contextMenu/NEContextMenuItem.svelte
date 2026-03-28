<script lang="ts">
    import type { Snippet } from "svelte";
	import type { Attachment } from 'svelte/attachments';

    const {
        label,
        onselect,
        onfocus,
        onunfocus,
        children
    }: {
        label: string,
        onselect?: () => void,
        onfocus?: (event: UIEvent) => void,
        onunfocus?: (event: UIEvent) => void,
        children?: Snippet
    } = $props();

    function focus(event: UIEvent) {
        if(onfocus) onfocus(event);
        focused = true;
        if(focusTimeout) clearTimeout(focusTimeout);

        unfocusOthers(true);
    }

    function unfocusOthers(fromSubmenu: boolean = false) {
        for(const sibling of element?.parentElement?.querySelectorAll("[data-context-menu-item]") || []) {
            if(sibling === element) continue;
            sibling.dispatchEvent(new CustomEvent('otherItemFocused', { detail: {
                fromSubmenu
            }}));
        }
    }

    export function otherItemFocused(event: CustomEvent<{ fromSubmenu: boolean }>) {
        focused = false;

        if(event.detail.fromSubmenu) {
            if(onunfocus) onunfocus(new UIEvent('blur'));
        } else {
            if(focusTimeout) clearTimeout(focusTimeout);
            focusTimeout = setTimeout(() => {
                if(!focused) {
                    if(onunfocus) onunfocus(new UIEvent('blur'));
                }
            }, 300);
        }
    }

    let focusTimeout: number | null = null;
    let focused: boolean = false;
    let element: HTMLButtonElement | null = $state(null);

    $effect(() => {
        if(!element) return;

        element.addEventListener('otherItemFocused', otherItemFocused as EventListener);
        return () => {
            element?.removeEventListener('otherItemFocused', otherItemFocused as EventListener);
        }
    });

    let contextMenuDepth = $state(0);
    const setDepth: Attachment = (element: Element) => {
        if(contextMenuDepth !== 0) return;
        if(!(element instanceof HTMLElement)) return;

        let depth = 0;
        let el: HTMLElement | null = element;
        while(el) {
            depth++;
            el = el.parentElement?.closest('[data-context-menu]') as HTMLElement;
        }
        contextMenuDepth = depth - 1;

        return () => {};
    };
</script>

<button
    {@attach setDepth}
    class="context-menu-item"
    bind:this={element}
    data-context-menu-item
    onclick={onselect}
    onmouseover={children ? focus : () => unfocusOthers()}
    onfocus={children && focus} onblur={onunfocus}
    style="z-index: {1000 - contextMenuDepth};"
>
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