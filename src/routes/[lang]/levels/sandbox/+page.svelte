<script lang="ts">
    import { t } from "$lib/translations";
    import { Weights } from "$lib/weights.svelte";
    import LevelComp from "$lib/components/Level.svelte";
    import Tetris from "$lib/components/Tetris.svelte";
    import WeightsControl from "$lib/components/WeightsControl.svelte";
    import Scoreboard from "$lib/components/Scoreboard.svelte";

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
            <label for="time-pressure"
                >{$t("feature_control.time_pressure")}</label
            >
        </div>
        <Scoreboard
            key="sandbox"
            bind:this={scoreboard}
            onWeightsSelect={(newWeights) => weights.setWeights(newWeights)}
        />
    {/snippet}
</LevelComp>
