<script lang="ts">
    import type { PageProps } from "./$types";
    import { locale } from "$lib/translations";
    import LevelComp from "$lib/Level.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";
    import Goals from "$lib/Goals.svelte";
    import { Weights } from "$lib/weights.svelte";
    import { LEVEL_INFO, LEVEL_CONFIG } from "$lib/levels";

    let { data }: PageProps = $props();

    let levelKey: string = $state(data.levelKey!);
    $effect(() => {
        levelKey = data.levelKey!;
    });

    let levelInfo = $derived(LEVEL_INFO[levelKey]);
    let levelConfig = $derived(LEVEL_CONFIG[levelKey]);

    let goals: Goals;
    let scoreboard: Scoreboard;
    let weights = $derived(
        new Weights(levelConfig.features, levelConfig.lockedFeatures),
    );
</script>

<LevelComp key={levelKey} title={levelInfo.name[$locale]}>
    {#snippet content()}
        <Tetris
            {weights}
            maxSpeed={9}
            onNewStats={(stats) => goals.updateGoals(stats)}
            onGameOver={(stats) => scoreboard.addEntry(stats, weights)}
        />
    {/snippet}
    {#snippet explanation()}
        {@html levelConfig.description[$locale]}
    {/snippet}
    {#snippet side()}
        <Goals bind:this={goals} goals={levelConfig.goals} />
        <WeightsControl {weights} />
        <Scoreboard
            bind:this={scoreboard}
            key={levelKey}
            onWeightsSelect={(newWeights) => weights.setWeights(newWeights)}
        />
    {/snippet}
</LevelComp>
