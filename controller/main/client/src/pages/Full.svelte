<script lang="ts">
    import LayoutTab from "./tabs/LayoutTab.svelte";
    import NodesTab from "./tabs/NodesTab.svelte";
    import OverviewTab from "./tabs/OverviewTab.svelte";
    import PresetsTab from "./tabs/PresetsTab.svelte";
    import SettingsTab from "./tabs/SettingsTab.svelte";
    
    const tabs: { name: string; component: any }[] = [
        { name: "Overview", component: OverviewTab },
        { name: "Layout", component: LayoutTab },
        { name: "Presets", component: PresetsTab },
        { name: "Nodes", component: NodesTab },
        { name: "Settings", component: SettingsTab }
    ];
    
    let activeTab = tabs[0];
</script>

<div class="content">
    <header>
        <h1>Full interface</h1>
        <a href="/">Simple interface</a>
    </header>
    <nav>
        {#each tabs as tab}
        <button
        class:active={tab === activeTab}
        on:click={() => activeTab = tab}
        >
        {tab.name}
    </button>
    {/each}
</nav>
<main>
    <svelte:component this={activeTab.component} />
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
        
        button {
            border: none;
            font-size: 1rem;
            padding: 0.4rem 1rem;
            text-decoration: none;
            font-weight: 400;
            color: var(--subtext0);
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
        
        &:has(button:hover) button:not(:hover) {
            color: var(--subtext1);
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
