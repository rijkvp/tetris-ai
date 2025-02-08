<script lang="ts">
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Tetris from "$lib/Tetris.svelte";

    let tetris: Tetris;

    let tickRateExp = $state(3);
    let tickRate = $derived(Math.pow(2, tickRateExp));
</script>

<h1>Tetris AI</h1>
<div class="panels">
    <Tetris bind:this={tetris} />
    <div>
        <h2>Speed</h2>
        <input
            id="speed-input"
            type="range"
            min="2"
            max="16"
            bind:value={tickRateExp}
            oninput={() => tetris.setSpeed(1 / tickRate)}
        />
        <label for="speed-input">{tickRate} tick/s</label>
        <WeightsControl
            onWeightsChange={(weights) => tetris.setWeights(weights)}
        />
    </div>
</div>

<style>
    .panels {
        display: flex;
        justify-content: space-between;
        gap: 20px;
    }
</style>
