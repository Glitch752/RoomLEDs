<script lang="ts">
    import { statusMessage } from "../../websocket";
    import { InterfaceTheme, theme } from "../../../settings.svelte";

    function capitalize(name: string) {
        return name.charAt(0).toUpperCase() + name.slice(1).toLowerCase();
    }
</script>

<div class="content">
    <section>
        <h1>Interface</h1>
        <select bind:value={$theme} id="themeSelector">
            {#each Object.values(InterfaceTheme) as theme}
                <option value={theme}>{capitalize(theme)}</option>
            {/each}
        </select>
    </section>

    <section>
        <h1>Status</h1>
        <p class="status">{@html $statusMessage}</p>
    </section>
</div>

<style lang="scss">
.content {
    text-align: left;
    padding: 0 1rem;
}
section {
    margin: 1rem 0;
    padding: 1rem;
    background-color: var(--surface0);
}
select {
    padding: 0.5rem;
    border: none;
    background-color: var(--surface1);
    color: var(--text);
    font-size: 1rem;
    cursor: pointer;
}
h1 {
    padding: 0;
    margin-bottom: 1rem;
}
p {
    padding: 0;
}
</style>