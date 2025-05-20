<script lang="ts">
    import type { Weights } from "$lib/weights.svelte";
    import { onMount } from "svelte";
    import { TetrisAnimator } from "$lib/animator.svelte";
    import { TetrisSimulator } from "$lib/simulator.svelte";
    import TetrisBoard from "$lib/components/TetrisBoard.svelte";

    let {
        weights = $bindable(),
        isRunning = $bindable(),
    }: { weights: Weights; isRunning: boolean } = $props();

    let tetrisBoard: TetrisBoard;
    let sim = new TetrisSimulator();
    let animator = new TetrisAnimator(
        sim,
        () => {
            tetrisBoard.display();
        },
        () => {
            restartSim();
        },
    );

    function restartSim() {
        sim.reset();
        tetrisBoard.clear();
        animator.restart();
        animator.setSpeed(50);
    }

    $effect(() => {
        sim.updateWeights(weights.getWeightsMap());
    });

    $effect(() => {
        animator.setRunning(isRunning);
    });

    onMount(() => {
        restartSim();

        return () => animator.stop();
    });
</script>

<TetrisBoard
    bind:this={tetrisBoard}
    bind:state={sim.state}
    bind:currentMove={animator.currentMove}
/>
