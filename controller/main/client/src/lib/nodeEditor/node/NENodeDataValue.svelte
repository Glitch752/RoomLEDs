<script lang="ts">
    import ColorPicker, { ChromeVariant } from "svelte-awesome-color-picker";
    import type { NodeDataValue, NodeDataValueDescriptor } from "../NodeVariants";

    const {
        key,
        descriptor,
        value,
        onchange
    }: {
        key: string,
        descriptor: NodeDataValueDescriptor,
        value: NodeDataValue,
        onchange: (newValue: NodeDataValue) => void
    } = $props();

    // Stop propegation so we don't prevent the event later
    const onmousedown = (e: MouseEvent) => e.stopPropagation();

    function hexToRgba(hex: string): { r: number; g: number; b: number; a: number } {
        let r = 0, g = 0, b = 0, a = 1;

        // Remove the leading '#' if present
        if (hex.startsWith('#')) {
            hex = hex.slice(1);
        }

        if (hex.length === 6) {
            r = parseInt(hex.slice(0, 2), 16);
            g = parseInt(hex.slice(2, 4), 16);
            b = parseInt(hex.slice(4, 6), 16);
        } else if (hex.length === 8) {
            r = parseInt(hex.slice(0, 2), 16);
            g = parseInt(hex.slice(2, 4), 16);
            b = parseInt(hex.slice(4, 6), 16);
            a = parseInt(hex.slice(6, 8), 16) / 255;
        }

        return { r, g, b, a };
    }
    function rgbaToHex(rgba: { r: number; g: number; b: number; a: number }): string {
        const rHex = rgba.r.toString(16).padStart(2, '0');
        const gHex = rgba.g.toString(16).padStart(2, '0');
        const bHex = rgba.b.toString(16).padStart(2, '0');
        const aHex = Math.round(rgba.a * 255).toString(16).padStart(2, '0');

        return `#${rHex}${gHex}${bHex}${aHex}`;
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="data-value">
    {#if descriptor.type === "boolean"}
        <label>
            <input type="checkbox" checked={value as boolean} onchange={(e) => {
                onchange((e.target as HTMLInputElement).checked);
            }} {onmousedown} />
            {descriptor.label}
        </label>
    {:else if descriptor.type === "number"}
        <label>
            {descriptor.label}
            <input type="number" value={value as number} onchange={(e) => {
                onchange(parseFloat((e.target as HTMLInputElement).value));
            }} {onmousedown} />
        </label>
    {:else if descriptor.type === "string"}
        <label>
            <input type="text" value={value as string} onchange={(e) => {
                onchange((e.target as HTMLInputElement).value);
            }} placeholder={descriptor.label} {onmousedown} />
        </label>
    {:else if descriptor.type === "enum"}
        <label>
            <select value={value as string} onchange={(e) => {
                onchange((e.target as HTMLSelectElement).value);
            }} title={descriptor.label} {onmousedown}>
                {#each descriptor.options as option}
                    <option value={option} selected={option === (value as string)}>{option}</option>
                {/each}
            </select>
        </label>
    {:else if descriptor.type === "color"}
        <label>
            <ColorPicker
                rgb={hexToRgba(value as string)}
                onInput={(color) => {
                    onchange(rgbaToHex(color.rgb!));
                }}
                components={ChromeVariant as any}
                sliderDirection="horizontal"
            />
        </label>
    {/if}
</div>

<style lang="scss">
.data-value {
    padding: 0.25rem 0.5rem;
    font-size: 0.85em;

    label {
        display: flex;
        align-items: center;
        gap: 6px;
        width: 100%;

        input[type="number"],
        input[type="text"],
        select {
            flex-grow: 1;
            border-radius: 4px;
            background-color: var(--surface2);
            border: 1px solid var(--surface1);
            color: var(--text);
            font-size: inherit;
            padding: 0 0.25rem;
            width: 100%;
        }
    }
}
</style>