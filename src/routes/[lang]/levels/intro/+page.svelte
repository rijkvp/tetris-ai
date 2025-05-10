<script lang="ts">
    import Level from "$lib/Level.svelte";
    import PlayableTetris from "$lib/PlayableTetris.svelte";
</script>

<Level key="intro">
    {#snippet content()}
        <PlayableTetris />
    {/snippet}
</Level>
