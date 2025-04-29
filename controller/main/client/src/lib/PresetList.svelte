<script lang="ts">
    import { runEffectPreset } from "../api/presets";
    import { presets } from "../websocket";
</script>

<ul>
    {#each $presets as preset}
        <li>
            <button class="gray" title={preset.name} onclick={() => runEffectPreset(preset.id)} aria-label={preset.name}>
                <i class={preset.icon}></i>
                <span>{preset.name}</span>
            </button>
        </li>
    {/each}
</ul>

<style lang="scss">
    button {
        display: inline-flex;
        align-items: center;
        gap: 1rem;
        
        font-size: 1.25rem;
        color: var(--text);
        cursor: pointer;
        padding: 0.5rem 0.75rem;

        i {
            font-size: 1.75rem;
        }
    }

    ul {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        padding: 0;
        margin: 0;
        
        li {
            list-style: none;
            margin: 0;
            padding: 0;
        }
    }

    @media (max-width: 700px) {
        ul {
            flex-direction: column;
        }
        button {
            width: 100%;
            justify-content: center;

            i {
                font-size: 1.5rem;
            }
        }
    }
</style>