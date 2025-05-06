<script lang="ts">
    import Level from "$lib/Level.svelte";
    import PlayableTetris from "$lib/PlayableTetris.svelte";
</script>

<Level key="intro">
    {#snippet content()}
        <PlayableTetris />
    {/snippet}
    {#snippet side()}
        <h2>Controls</h2>
        <ul>
            <li>
                <strong>Left/Right arrow keys</strong>: move the tetromino to
                the left/right
            </li>
            <li>Up arrow/Z: Rotate the tetromino</li>
            <li>Down arrow: Move the tetromino down (soft drop)</li>
            <li>Space: Drop the tetromino (hard drop)</li>
        </ul>
    {/snippet}
</Level>
