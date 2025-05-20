<script lang="ts">
    import type { PageProps } from "./$types";
    import { Weights } from "$lib/weights.svelte";
    import { LEVEL_CONFIG } from "$lib/levels";
    import Level from "$lib/components/Level.svelte";
    import Tetris from "$lib/components/Tetris.svelte";
    import WeightsControl from "$lib/components/WeightsControl.svelte";
    import Scoreboard from "$lib/components/Scoreboard.svelte";
    import Goals from "$lib/components/Goals.svelte";

    let { data }: PageProps = $props();

    let levelKey: string = $state(data.levelKey!);
    $effect(() => {
        levelKey = data.levelKey!;
    });

    let levelConfig = $derived(LEVEL_CONFIG[levelKey]);

    let goals: Goals;
    let scoreboard: Scoreboard;
    let weights = $derived(
        new Weights(levelConfig.features, levelConfig.lockedFeatures),
    );
</script>

<Level key={levelKey}>
    {#snippet content()}
        <Tetris
            {weights}
            maxSpeed={6}
            onNewStats={(stats) => goals.updateGoals(stats)}
            onGameOver={(stats) => scoreboard.addEntry(stats, weights)}
        />
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
</Level>
