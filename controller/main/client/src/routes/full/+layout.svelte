<script lang="ts">
    import { page } from "$app/state";
    import type { Snippet } from "svelte";

    const { children }: { children: Snippet } = $props();
    
    const tabs: { name: string; href: string }[] = [
        { name: "Overview", href: "/full/overview" },
        { name: "Layout", href: "/full/layout" },
        { name: "Presets", href: "/full/presets" },
        { name: "Nodes", href: "/full/nodes" },
        { name: "Settings", href: "/full/settings" }
    ];
</script>

<div class="content">
    <header>
        <h1>Full interface</h1>
        <a href="/">Simple interface</a>
    </header>
    <nav>
        {#each tabs as tab}
            <a
                class:active={page.url.pathname === tab.href}
                href={tab.href}
            >
                {tab.name}
            </a>
    {/each}
</nav>
<main>
    {@render children()}
</main>
</div>

<style lang="scss">
.content {
    display: grid;
    width: 100%;
    height: 100vh;
    
    grid-template-rows: 2rem minmax(0, 1fr);
    grid-template-columns: 10rem minmax(0, 1fr);
}

header {
    grid-row: 1 / 2;
    grid-column: 1 / 3;
    
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--dark-bg);
    color: white;
    border-bottom: 2px solid var(--contrast-border);
    
    h1 {
        margin: 0;
        padding: 0;
        margin-left: 1rem;
        font-size: 1.25rem;
        font-weight: 500;
    }
    
    a {
        margin-right: 1rem;
        color: var(--subtext1);
        text-decoration: none;
    }
}

nav {
    grid-row: 2 / 3;
    grid-column: 1 / 2;
    
    display: flex;
    flex-direction: column;
    background-color: var(--dark-bg);
    color: var(--text);
    border-right: 2px solid var(--contrast-border);
    
    a {
        display: block;
        border: none;
        font-size: 1rem;
        padding: 0.4rem 1rem;
        font-weight: 400;
        color: var(--subtext0);
        text-decoration: none;
        background-color: var(--dark-bg);
        transition: background-color 0.2s, color 0.2s, padding-right 0.2s, border-color 0.2s;
        text-align: left;
        border-left: 4px solid;
        border-color: transparent;
        
        &.active {
            background-color: var(--background);
            border-color: var(--sapphire);
            color: var(--text);
        }
        
        &:hover {
            color: var(--text);
        }
    }
}

main {
    grid-row: 2 / 3;
    grid-column: 2 / 3;
    
    display: flex;
    flex-direction: column;
    overflow: hidden;
}
</style>
