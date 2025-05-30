<script lang="ts">
    import { Game } from "tetris-ai";
    import { onDestroy, onMount } from "svelte";
    import GameControls from "$lib/components/GameControls.svelte";
    import TetrisBoard from "$lib/components/TetrisBoard.svelte";
    import StatsPanel from "$lib/components/StatsPanel.svelte";

    let game: Game = new Game();
    let tetrisBoard: TetrisBoard;

    let gameState = $state(game.state);
    let currentMove = $state(game.move ?? null);
    let isRunning = $state(true);
    let gameOver = $state(false);

    let animationFrame: number;
    let lastFrameTime = 0;
    const TICK_DURATION = 0.4;
    let timer = TICK_DURATION;

    function gameLoop(currentTime: number) {
        if (!isRunning) return;

        // ensure deltaTime is at least 1ms, to avoid division by zero or negative values
        const deltaTime = Math.max(currentTime - lastFrameTime, 1) / 1000;
        lastFrameTime = currentTime;
        animationFrame = requestAnimationFrame(gameLoop);

        // update
        timer -= deltaTime;
        if (timer <= 0) {
            if (!game.step()) {
                gameOver = true;
                isRunning = false;
            }
            timer = TICK_DURATION;
        }

        // display
        gameState = game.state;
        currentMove = game.move ?? null;
        tetrisBoard.display();
    }

    function togglePaused() {
        isRunning = !isRunning;
        if (isRunning) {
            startGameLoop();
        }
    }

    function startGameLoop() {
        lastFrameTime = performance.now(); // prevents time from 'ticking' while paused
        animationFrame = requestAnimationFrame(gameLoop);
    }

    function newGame() {
        game.reset();
        tetrisBoard.clear();
        gameOver = false;
        isRunning = true;
        startGameLoop();
    }

    function handleKeydown(event: KeyboardEvent) {
        // Allow repeating keys, uncomment to disable
        // if (event.repeat) return;
        if (event.key === "l" || event.key === "ArrowRight") {
            game.move_right();
        } else if (event.key === "j" || event.key === "ArrowLeft") {
            game.move_left();
        } else if (event.key === "k" || event.key === "ArrowDown") {
            game.soft_drop();
        } else if (event.key === "z" || event.key === "ArrowUp") {
            game.rotate();
        } else if (event.key === " ") {
            event.preventDefault();
            game.hard_drop();
        }
    }

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
        newGame();
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
        isRunning = false;
        window.cancelAnimationFrame(animationFrame);
    });
</script>

<div class="grid">
    <div class="stats">
        <StatsPanel bind:stats={gameState.stats} />
    </div>
    <div class="controls">
        <GameControls
            bind:isRunning
            bind:gameOver
            onPauseToggle={() => togglePaused()}
            onNewGame={() => newGame()}
        />
    </div>
    <div class="board-frame">
        <TetrisBoard
            bind:this={tetrisBoard}
            bind:state={gameState}
            bind:currentMove
        />
    </div>
</div>

<style>
    .grid {
        display: grid;
        grid-template-columns: auto auto;
        grid-template-rows: min-content auto;
        gap: 16px;
    }
    .board-frame {
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
