<script lang="ts">
    import { onMount } from "svelte";
    import TetrisBoard from "./TetrisBoard.svelte";
    import { TetrisAnimator } from "./animator.svelte";
    import { TetrisSimulator } from "./simulator.svelte";
    import type { Weights } from "./weights.svelte";

    let { weights = $bindable() }: { weights: Weights } = $props();

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
