<script lang="ts">
    import type { PageProps } from "./$types";
    import type { Level } from "$lib/levels";
    import { locale } from "$lib/translations";
    import LevelComp from "$lib/Level.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";
    import Goals from "$lib/Goals.svelte";

    let { data }: PageProps = $props();
    const level: Level = data.level!;

    let goals: Goals;
    let scoreboard: Scoreboard;
    let tetris: Tetris;
</script>

<LevelComp title={level.name[$locale]}>
    {#snippet content()}
        <Tetris
            bind:this={tetris}
            maxSpeed={level.goals != null ? 8 : 12}
            onNewStats={(stats) => goals.updateGoals(stats)}
            onGameOver={(stats) => scoreboard.addEntry(stats)}
        />
    {/snippet}
    {#snippet explanation()}
        {@html level.description[$locale]}
    {/snippet}
    {#snippet side()}
        <Goals bind:this={goals} goals={level.goals} />
        <WeightsControl
            onWeightsChange={(weights) => tetris.setWeights(weights)}
            availableFeatures={level.features}
        />
        <Scoreboard key={level.key} bind:this={scoreboard} />
    {/snippet}
</LevelComp>
