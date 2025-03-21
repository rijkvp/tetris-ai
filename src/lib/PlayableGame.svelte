<script lang="ts">
    import { t, locale } from "$lib/translations";
    import PlayableTetris from "$lib/PlayableTetris.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";

    let scoreboard: Scoreboard;
</script>

<div class="panels">
    <PlayableTetris />
    <div class="panel">
        <div>
            <h1>Playable Tetris</h1>
            <p>Play the game here</p>
        </div>
        <div>
            <Scoreboard key="playable" bind:this={scoreboard} />
        </div>
    </div>
</div>

<style>
    .panels {
        display: flex;
        flex-wrap: wrap;
        justify-content: space-between;
        gap: 2rem;
    }
    .panel {
        /* fill up the remaining space */
        flex-grow: 1;
        flex-shrink: 1;
        flex-basis: 0;

        display: flex;
        flex-direction: column;
        flex-basis: min-content;
        gap: 1rem;
    }
</style>
