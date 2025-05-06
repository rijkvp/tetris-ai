<script lang="ts">
    import Level from "$lib/Level.svelte";
    import TetrisTrainer from "$lib/TetrisTrainer.svelte";
</script>

<Level key="train">
    {#snippet content()}
        <TetrisTrainer />
    {/snippet}
    {#snippet side()}{/snippet}
</Level>
