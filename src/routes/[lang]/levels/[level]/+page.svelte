<script lang="ts">
    import type { PageProps } from "./$types";
    import type { Level } from "$lib/levels";
    import { locale } from "$lib/translations";
    import LevelComp from "$lib/Level.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";
    import Goals from "$lib/Goals.svelte";
    import { Weights } from "$lib/weights.svelte";

    let { data }: PageProps = $props();

    let level: Level = $state(data.level!);
    $effect(() => {
        level = data.level!;
    });

    let goals: Goals;
    let scoreboard: Scoreboard;
    let weights = $derived(new Weights(level.features));
</script>

<LevelComp key={level.key} title={level.name[$locale]}>
    {#snippet content()}
        <Tetris
            {weights}
            maxSpeed={level.goals != null ? 8 : 12}
            onNewStats={(stats) => goals.updateGoals(stats)}
            onGameOver={(stats) => scoreboard.addEntry(stats, weights)}
        />
    {/snippet}
    {#snippet explanation()}
        {@html level.description[$locale]}
    {/snippet}
    {#snippet side()}
        <Goals bind:this={goals} goals={level.goals} />
        <WeightsControl {weights} />
        <Scoreboard
            bind:this={scoreboard}
            key={level.key}
            onWeightsSelect={(newWeights) => weights.setWeights(newWeights)}
        />
    {/snippet}
</LevelComp>
