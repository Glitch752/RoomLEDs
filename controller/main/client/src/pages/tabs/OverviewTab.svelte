<script lang="ts">
    import LightVisualization from "../../lib/LightVisualization.svelte";
  import PresetList from "../../lib/PresetList.svelte";
</script>

<div class="content">
    <h1>Presets</h1>
    <PresetList />
    
    <br />
    <br />
    <LightVisualization />
</div>
    
<style lang="scss">
    h1 {
        margin: 0;
        padding: 0;
        text-align: left;
        font-size: 1.5rem;
    }

    .content {
        padding: 1rem 3rem 3rem 3rem;
        overflow-y: auto;
    }
</style>