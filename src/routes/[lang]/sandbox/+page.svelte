<script lang="ts">
    import { locale } from "$lib/translations";
    import LevelComp from "$lib/Level.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";

    let scoreboard: Scoreboard;
    let tetris: Tetris;

    const level = {
        key: "sandbox",
        name: { en: "Sandbox", nl: "Sandbox-modus" },
        description: {
            en: "Play around with all the features.",
            nl: "Speel met alle kenmerken.",
        },
    };
</script>

<LevelComp title={level.name[$locale]}>
    {#snippet content()}
        <Tetris
            bind:this={tetris}
            onGameOver={(stats) => scoreboard.addEntry(stats)}
        />
    {/snippet}
    {#snippet side()}
        <p>
            {@html level.description[$locale]}
        </p>
        <WeightsControl
            onWeightsChange={(weights) => tetris.setWeights(weights)}
        />
        <Scoreboard key={level.key} bind:this={scoreboard} />
    {/snippet}
</LevelComp>
