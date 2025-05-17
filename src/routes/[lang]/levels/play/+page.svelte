<script lang="ts">
    import Level from "$lib/Level.svelte";
    import PlayableTetris from "$lib/PlayableTetris.svelte";
</script>

<Level key="play">
    {#snippet content()}
        <PlayableTetris />
    {/snippet}
</Level>
