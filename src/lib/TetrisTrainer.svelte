<script lang="ts">
    import { Trainer } from "tetris-ai";
    import TetrisBoard from "$lib/TetrisBoard.svelte";
    import StatsPanel from "$lib/StatsPanel.svelte";
    import { onDestroy, onMount } from "svelte";
    import GameControls from "./GameControls.svelte";

    let trainer: Trainer = Trainer.from_feature_names([
        "col_trans",
        "row_trans",
    ]);
    let tetrisBoard: TetrisBoard;

    // let gameState = $state(game.state);
    // let currentMove = $state(game.move ?? null);
    let isRunning = $state(true);
    let gameOver = $state(false);

    let animationFrame: number;
    let lastFrameTime = 0;
    const TICK_DURATION = 0.4;
    let timer = TICK_DURATION;

    function togglePaused() {
        isRunning = !isRunning;
        if (isRunning) {
            startGameLoop();
        }
    }

    function startGameLoop() {
        console.log("Animating frame");
        const startTime = performance.now();
        let state = trainer.train_gen();
        const endTime = performance.now();
        console.log("Frame time:", endTime - startTime);
        console.log(state);
    }

    function newGame() {
        // tetrisBoard.clear();
        gameOver = false;
        isRunning = true;
        startGameLoop();
    }
</script>

<div class="grid">
    <div class="controls">
        <GameControls
            bind:isRunning
            bind:gameOver
            onPauseToggle={() => togglePaused()}
            onNewGame={() => newGame()}
        />
    </div>
    <div class="board"></div>
</div>

<style>
    .grid {
        display: grid;
        grid-template-columns: auto auto;
        grid-template-rows: min-content auto;
        gap: 16px;
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
</style>
