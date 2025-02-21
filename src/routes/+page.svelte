<script lang="ts">
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";
    import type { Stats } from "$lib/types";

    let tetris: Tetris;
    let scoreboard: Scoreboard;

    let tickRateExp = $state(3);
    let tickRate = $derived(Math.pow(2, tickRateExp));
</script>

<h1>Tetris AI</h1>
<div class="panels">
    <Tetris
        bind:this={tetris}
        onGameOver={(stats: Stats) => scoreboard.addEntry(stats)}
    />
    <div>
        <h2>Speed</h2>
        <input
            id="speed-input"
            type="range"
            min="0"
            max="20"
            bind:value={tickRateExp}
            oninput={() => tetris.setSpeed(1 / tickRate)}
        />
        <label for="speed-input">{tickRate} tick/s</label>
        <WeightsControl
            onWeightsChange={(weights) => tetris.setWeights(weights)}
        />
        <Scoreboard bind:this={scoreboard} />
    </div>
</div>

<style>
    .panels {
        display: flex;
        justify-content: space-between;
        gap: 20px;
    }
</style>
