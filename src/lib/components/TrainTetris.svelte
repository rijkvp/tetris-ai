<script lang="ts">
    import type { Weights } from "$lib/weights.svelte";
    import { onMount } from "svelte";
    import { TetrisAnimator } from "$lib/animator.svelte";
    import { TetrisSimulator } from "$lib/simulator.svelte";
    import TetrisBoard from "$lib/components/TetrisBoard.svelte";

    let { isRunning = $bindable() }: { isRunning: boolean } = $props();

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
    let weights: Weights | null = null;

    $effect(() => {
        animator.setRunning(isRunning);
    });

    function restartSim() {
        tetrisBoard.clear();
        sim.reset();
        if (weights != null) {
            sim.updateWeights(weights.getWeightsMap());
        }
        animator.restart();
        animator.setSpeed(50);
    }

    export const updateWeights = (newWeights: Weights) => {
        weights = newWeights;
        restartSim();
    };

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
