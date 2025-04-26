<!--
@component
A component that allows the user to select a Font Awesome icon from a dropdown list, including search functionality.
-->

<script lang="ts">
  import { onMount } from "svelte";
    import { slide } from "svelte/transition";
    // We use a dynamic import to avoid including the icon data in the initial bundle
    const iconsPromise = import("./iconData.compile").then(data => data.icons);

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
    let dropdownOpen = $state(false);
    let icons: Awaited<typeof iconsPromise> = $state([]);
    
    let iconData = $derived.by(() => {
        const parts = value.split(" ");
        const name = parts[parts.length - 1].replace("fa-", "");
        return icons.find(icon => icon.name === name);
    });

    const filteredIcons = $derived.by(() => {
        if(!search) return icons;
        const searchLower = search.toLowerCase();
        
        // If the icon name or any of its search terms contain the name
        return icons.filter(icon => {
            const iconName = icon.name.toLowerCase();
            return (
                iconName.includes(searchLower) ||
                icon.searchTerms.some(term => term.includes(searchLower))
            );
        });
    });
    
    function selectIcon(icon: string) {
        value = icon;
    }

    // When clicking outside of the dropdown, close it
    function handleClickOutside(event: MouseEvent) {
        const target = event.target as HTMLElement;
        if(!dropdownOpen || target.closest(".iconSelector")) return;
        dropdownOpen = false;
    }

    // Load the icons data when the component is mounted
    onMount(async () => {
        try {
            icons = await iconsPromise;
        } catch (error) {
            console.error("Failed to load icons data:", error);
        }
    });
</script>

<svelte:window onclick={handleClickOutside} />
<div class="iconSelector">
    <button class="selector" onclick={() => dropdownOpen = !dropdownOpen} aria-label={ariaLabel}>
        <i class={`icon ${value}`}></i>
        <span class="iconName">{iconData?.name ?? "Unknown"}</span>
        {#if dropdownOpen}
            <i class="openIcon fas fa-caret-down"></i>
        {:else}
            <i class="openIcon fas fa-caret-right"></i>
        {/if}
    </button>

    {#if dropdownOpen}
        <div class="dropdown" transition:slide={{ duration: 200 }} aria-expanded={dropdownOpen}>
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
                            selectIcon(selectedIcon.class);
                        }
                    }
                }}
            />
            
            <div class="icons">
                {#each filteredIcons as icon}
                    <button class="unicodeIcon" title={icon.name} onclick={() => selectIcon(icon.class)}>{icon.unicode}</button>
                {/each}
                {#if filteredIcons.length === 0}
                    <p>No icons found</p>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .iconSelector {
        position: relative;
        display: inline-block;
    }

    .selector {
        padding: 0.5rem 1rem;
        display: flex;
        align-items: center;
        gap: 1rem;
        background-color: #171719;
        border: none;
        color: white;
    }
    .icon {
        font-size: 1.5rem;
    }
    .openIcon {
        font-size: 1.25rem;
        color: #eee;
    }
    .iconName {
        font-size: 1.25rem;
        color: #eee;
    }

    .dropdown {
        position: absolute;
        background-color: #111111;
        width: 37rem;
        max-height: 30rem;
        z-index: 10;

        display: flex;
        flex-direction: column;
    }

    input {
        padding: 0.5rem;
        background-color: #252529;
        color: #fff;
        border: none;
        outline: none;
        margin: 1rem;
        font-size: 1rem;
    }

    .icons {
        display: flex;
        flex-wrap: wrap;
        gap: 0rem;
        line-height: 1;
        padding: 0 0.5rem 1rem 0.5rem;
        overflow-y: auto;
    }

    .unicodeIcon {
        font: 1.9rem "Font Awesome 6 Free";
        font-weight: 900;

        border: none;
        width: 1.25lh;
        background: none;
        text-align: center;
        color: #bbb;
        transition: color 0.2s;
    }
    .unicodeIcon:hover {
        color: #fff;
        cursor: pointer;
    }

    p {
        color: #ccc;
        padding: 0;
        margin: 0;
        text-align: center;
    }
</style>
