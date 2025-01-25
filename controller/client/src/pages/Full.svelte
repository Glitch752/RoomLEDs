<script lang="ts">
import OverviewTab from "./tabs/OverviewTab.svelte";
import SettingsTab from "./tabs/SettingsTab.svelte";

const tabs: { name: string; component: any }[] = [
  { name: "Overview", component: OverviewTab },
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

  grid-template-rows: 3rem 1fr;
  grid-template-columns: 25rem 1fr;
}

header {
  grid-row: 1 / 2;
  grid-column: 1 / 3;
  
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #111;
  color: white;
  border-bottom: 2px solid black;
  
  h1 {
    margin: 0;
    padding: 0;
    margin-left: 1rem;
    font-weight: 400;
    font-size: 1.75rem;
  }

  a {
    margin-right: 1rem;
    color: #ccc;
    text-decoration: none;
  }
}

nav {
  grid-row: 2 / 3;
  grid-column: 1 / 2;
  
  display: flex;
  flex-direction: column;
  background-color: #111;
  color: white;

  border-right: 2px solid black;
  
  button {
    border: none;
    outline: none;
    font-size: 1.25rem;
    padding: 0.5rem 1rem;
    text-decoration: none;
    color: #ddd;
    background-color: #111;
    transition: background-color 0.2s, color 0.2s, padding-right 0.2s;

    &.active {
      background-color: #151515;
      color: #fff;
    }
    
    &:hover {
      background-color: #181818;
      color: #fff;
    }
  }

  &:has(button:hover) button:not(:hover) {
    color: #aaa;
    padding-right: 1.5rem;
  }
}

main {
  grid-row: 2 / 3;
  grid-column: 2 / 3;
  
  padding: 3rem;

  overflow-y: auto;
}
</style>
