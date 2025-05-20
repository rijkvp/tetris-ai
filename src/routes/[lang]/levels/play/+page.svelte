<script lang="ts">
    import Level from "$lib/components/Level.svelte";
    import PlayableTetris from "$lib/components/PlayableTetris.svelte";
</script>

<Level key="play">
    {#snippet content()}
        <PlayableTetris />
    {/snippet}
</Level>
