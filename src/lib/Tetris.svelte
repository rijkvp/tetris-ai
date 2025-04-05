<script lang="ts">
    import { t } from "$lib/translations";
    import { onDestroy } from "svelte";
    import { TetrisAnimator } from "$lib/animator.svelte";
    import { TetrisSimulator } from "$lib/simulator.svelte";
    import type { Stats } from "$lib/types.ts";
    import TetrisBoard from "$lib/TetrisBoard.svelte";
    import StatsPanel from "$lib/StatsPanel.svelte";
    import GameControls from "$lib/GameControls.svelte";
    import { onMount } from "svelte";
    import type { Weights } from "./weights.svelte";

    const SPEED_MUTIPLIER = [
        0.2, 0.5, 1, 2, 5, 10, 20, 50, 100, 1000, 10000, 100000, 1000000,
    ];

    let {
        weights,
        onNewStats,
        onGameOver,
        maxSpeed = SPEED_MUTIPLIER.length - 1,
    }: {
        weights: Weights;
        onNewStats?: (stats: Stats) => void;
        onGameOver: (stats: Stats) => void;
        maxSpeed?: number;
    } = $props();

    let sim = new TetrisSimulator();
    let animator = new TetrisAnimator(
        sim,
        () => tetrisBoard.display(),
        () => onGameOver(sim.state.stats),
    );

    $effect(() => {
        sim.updateWeights(weights.getWeightsMap());
    });

    // TODO: fix ugly hacks
    $effect(() => {
        if (animator.isRunning) {
            onNewStats?.(sim.state.stats);
        }
    });

    let speedIndex = $state(2);
    let speedMultiplier = $derived(SPEED_MUTIPLIER[speedIndex]);
    $effect(() => {
        animator.setSpeed(speedMultiplier);
    });

    let moves = 0;
    let lastMoves = 0;
    let moveRate = $state(0);

    let tetrisBoard: TetrisBoard;
    const moveTimer = setInterval(() => {
        moveRate = moves - lastMoves;
        lastMoves = moves;
    }, 1000);

    function newGame() {
        sim.reset();
        tetrisBoard.clear();
        animator.restart();
    }

    onMount(() => {
        newGame();
    });

    onDestroy(() => {
        animator.stop();
        clearInterval(moveTimer);
    });
</script>

<div class="grid">
    <div class="stats">
        <StatsPanel bind:stats={sim.state.stats} bind:moveRate />
    </div>
    <div class="controls">
        <GameControls
            bind:isRunning={animator.isRunning}
            bind:gameOver={sim.state.gameOver}
            onPauseToggle={() => animator.togglePaused()}
            onNewGame={() => newGame()}
        />
        <div class="speed-control">
            <label for="speed">{$t("controls.speed")}</label>
            <input
                class="speed-input"
                title={$t("controls.speed")}
                name="speed"
                type="range"
                min="0"
                max={Math.min(SPEED_MUTIPLIER.length - 1, maxSpeed)}
                bind:value={speedIndex}
            />
            <span class="speed-display"
                >{speedMultiplier.toLocaleString()}x</span
            >
        </div>
    </div>
    <div class="board">
        <TetrisBoard
            bind:this={tetrisBoard}
            bind:state={sim.state}
            bind:currentMove={animator.currentMove}
        />
    </div>
</div>

<style>
    .grid {
        display: grid;
        grid-template-columns: min-content min-content;
        grid-template-rows: min-content 1fr;
        row-gap: 0.5rem;
        column-gap: 1rem;
    }
    .board {
        grid-column: 2;
        grid-row: 2;
    }
    .stats {
        grid-column: 1;
        grid-row: 2;
    }
    .controls {
        grid-column: 2;
        grid-row: 1;
        display: flex;
        flex-direction: column;
    }
    .controls > div {
        width: 100%;
        justify-content: center;
        display: flex;
        gap: 12px;
        height: fit-content;
    }
    .speed-input {
        width: 80%;
    }
    .speed-display {
        width: 20%;
    }
    .speed-control {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-top: 0.4rem;
    }
    .speed-control input {
        width: 8rem;
    }
</style>
