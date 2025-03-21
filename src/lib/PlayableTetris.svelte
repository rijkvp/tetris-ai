<script lang="ts">
    import { t } from "$lib/translations";
    import { Game } from "tetris-ai";
    import TetrisBoard from "$lib/TetrisBoard.svelte";
    import StatsPanel from "$lib/StatsPanel.svelte";
    import { onDestroy, onMount } from "svelte";

    let game: Game = new Game();
    let tetrisBoard: TetrisBoard;
    let statsPanel: StatsPanel = $state()!;

    let isRunning = $state(false);
    let gameOver = $state(false);

    let lastFrameTime = 0;
    const TICK_DURATION = 0.4;
    let timer = TICK_DURATION;

    function gameLoop(currentTime: number) {
        if (!isRunning) return;

        // ensure deltaTime is at least 1ms, to avoid division by zero or negative values
        const deltaTime = Math.max(currentTime - lastFrameTime, 1) / 1000;
        lastFrameTime = currentTime;
        requestAnimationFrame(gameLoop);

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
        tetrisBoard.display(game.state, game.move ?? null);
    }

    function reset() {
        game.reset();
        tetrisBoard.clear();
        gameOver = false;
    }

    function togglePaused() {
        isRunning = !isRunning;
        if (isRunning) {
            // start the game loop / animation
            lastFrameTime = performance.now(); // prevents time from 'ticking' while paused
            requestAnimationFrame(gameLoop);
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.repeat) {
            return;
        }
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
        reset();
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
    });
</script>

<div class="grid">
    <div class="stats">
        <StatsPanel bind:this={statsPanel} />
    </div>
    <div class="controls">
        <div>
            <button
                onclick={() => reset()}
                disabled={isRunning}
                title={$t("controls.reset")}
            >
                {$t("controls.reset")}
            </button>
            <button
                onclick={() => togglePaused()}
                disabled={gameOver}
                title={isRunning ? $t("controls.pause") : $t("controls.play")}
            >
                {#if isRunning}
                    Pause
                {:else}
                    Play
                {/if}
            </button>
        </div>
    </div>
    <div class="board">
        <TetrisBoard bind:this={tetrisBoard} bind:statsPanel />
    </div>
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
    .controls > div {
        width: 100%;
        justify-content: center;
        display: flex;
        gap: 12px;
        height: fit-content;
    }
    .controls button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 4rem;
        height: 1.5rem;
        font-size: 1.2rem;
    }
</style>
