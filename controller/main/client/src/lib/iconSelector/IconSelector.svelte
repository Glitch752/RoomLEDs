<!--
@component
A component that allows the user to select a Font Awesome icon from a dropdown list, including search functionality.
-->

<!-- TODO -->

<script module>
    // Parse the Font Awesome icon metadata into a simple list
    import metadata from "./icons.json";
    
    // TODO: Cache this or compute it on the server? This is a lot of JSON to parse.
    const icons = Object.entries(metadata).map(([key, value]) => {
        return {
            name: key,
            prefix: value.styles[0],
            unicode: value.unicode,
            label: `${value.styles[0]} ${key}`
        };
    });
</script>

<script lang="ts">
    let {
        value = $bindable(),
        placeholder = "Select an icon",
        ariaLabel = "Icon selector"
    }: {
        value: string;
        placeholder: string;
        ariaLabel: string;
    } = $props();

    let search = $state("");

    const filteredIcons = $derived.by(() => {
        if (!search) return icons;
        const searchLower = search.toLowerCase();
        return icons.filter(icon => icon.label.toLowerCase().includes(searchLower));
    });
    
    function selectIcon(icon: string) {
        value = icon;
    }
</script>

<span>
    <i class={value} aria-hidden="true"></i>
    <input
        type="text"
        bind:value={search}
        placeholder={placeholder}
        aria-label={ariaLabel}
        class="icon-search"
        oninput={() => {
            search = search.trim();
        }}
        onkeydown={e => {
            if (e.key === "Enter") {
                const selectedIcon = filteredIcons[0];
                if (selectedIcon) {
                    selectIcon(selectedIcon.label);
                }
            }
        }}
    />
    <div>
        {#each filteredIcons as icon}
            <!-- TODO: We can probably use the unicode value here to improve performance? -->
            <i class={icon.label}></i>
        {/each}
    </div>
</span>

<style>
    
</style>
