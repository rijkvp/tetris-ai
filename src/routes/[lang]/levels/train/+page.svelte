<script lang="ts">
    import Level from "$lib/Level.svelte";
    import TetrisTrainer from "$lib/TetrisTrainer.svelte";
</script>

<Level key="train" title="Train">
    {#snippet content()}
        <TetrisTrainer />
    {/snippet}
    {#snippet explanation()}
        <h2>Watch the AI being trained!</h2>
    {/snippet}
    {#snippet side()}
        TODO: Put weights here
    {/snippet}
</Level>
