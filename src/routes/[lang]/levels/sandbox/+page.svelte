<script lang="ts">
    import LevelComp from "$lib/Level.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";
    import { Weights } from "$lib/weights.svelte";

    let scoreboard: Scoreboard;
    let weights = new Weights();
    let timePressure = $state(true);
</script>

<LevelComp key="sandbox">
    {#snippet content()}
        <Tetris
            {weights}
            onGameOver={(stats) => scoreboard.addEntry(stats, weights)}
            bind:timePressure
        />
    {/snippet}
    {#snippet side()}
        <WeightsControl {weights} />
        <div>
            <input
                name="time-pressure"
                type="checkbox"
                bind:checked={timePressure}
            />
            <label for="time-pressure">Time pressure</label>
        </div>
        <Scoreboard
            key="sandbox"
            bind:this={scoreboard}
            onWeightsSelect={(newWeights) => weights.setWeights(newWeights)}
        />
    {/snippet}
</LevelComp>
