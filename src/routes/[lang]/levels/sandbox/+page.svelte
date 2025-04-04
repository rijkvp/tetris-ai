<script lang="ts">
    import { locale } from "$lib/translations";
    import LevelComp from "$lib/Level.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";
    import { Weights } from "$lib/weights.svelte";

    const level = {
        key: "sandbox",
        name: { en: "Sandbox", nl: "Sandbox-modus" },
        description: {
            en: "Play around with all the features.",
            nl: "Speel met alle kenmerken.",
        },
    };

    let scoreboard: Scoreboard;
    let weights = new Weights();
</script>

<LevelComp title={level.name[$locale]} key="sandbox">
    {#snippet content()}
        <Tetris
            {weights}
            onGameOver={(stats) => scoreboard.addEntry(stats, weights)}
        />
    {/snippet}
    {#snippet side()}
        <p>
            {@html level.description[$locale]}
        </p>
        <WeightsControl {weights} />
        <Scoreboard
            key={level.key}
            bind:this={scoreboard}
            onWeightsSelect={(newWeights) => weights.setWeights(newWeights)}
        />
    {/snippet}
</LevelComp>
