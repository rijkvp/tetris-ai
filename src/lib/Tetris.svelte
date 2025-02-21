<script lang="ts">
    import { Simulator, Weights } from "tetris-ai";
    import type { GameState, Stats } from "$lib/types.ts";
    import TetrisBoard from "$lib/TetrisBoard.svelte";
    import { onMount } from "svelte";

    let simulator: Simulator = new Simulator();

    let { onGameOver }: { onGameOver: (stats: Stats) => void } = $props();

    let tetrisBoard: TetrisBoard;
    let isRunning = $state(false);
    let gameOver = $state(false);

    let curr: GameState, next: GameState;

    let lastFrameTime = 0;
    let tick = 0;
    let tickTimer = 0;
    let tickInterval = 1 / 8;

    let displayMoves = $state(0);
    let moves = 0;
    let lastMoves = 0;

    setInterval(() => {
        displayMoves = moves - lastMoves;
        lastMoves = moves;
    }, 1000);

    function calcNext(): boolean {
        curr = next;
        if (!simulator.step()) {
            // game over
            gameOver = true;
            onGameOver(curr.stats);
            isRunning = false;
            return false;
        }
        moves++;
        next = simulator.state;
        return true;
    }

    function step() {
        calcNext();
        tetrisBoard.display(curr);
    }

    function animateFrame(deltaTime: number) {
        // check if frame is over
        if (tick > next.move.path.length) {
            calcNext();
            tick = 0;
            tickTimer = 0;
        }

        // update tick timers
        // ensure deltaTime is at least 1ms, to avoid division by zero or negative values
        tickTimer += deltaTime;
        while (tickTimer >= tickInterval) {
            tickTimer -= tickInterval;
            tick++;
        }
        const tickProgress = tickTimer / tickInterval; // progress from 0 to 1 within a tick

        tetrisBoard.displayTransition(curr, next, tick, tickProgress);
    }

    const targetFPS = 60;
    const maxFrameDuration = 1 / targetFPS / 2; // divide by 2 to allow time for drawing

    function gameLoop(currentTime: number) {
        const deltaTime = Math.max(currentTime - lastFrameTime, 1) / 1000;
        lastFrameTime = currentTime;
        console.log(`Delta time: ${deltaTime}, FPS: ${1 / deltaTime}`);
        if (isRunning) {
            if (tickInterval * next.move.path.length < deltaTime) {
                let timeSpent = 0;
                while (timeSpent < maxFrameDuration) {
                    // TODO: This makes no sense
                    timeSpent += tickInterval * next.move.path.length;
                    if (!calcNext()) {
                        break;
                    }
                }
                tetrisBoard.display(curr);
            } else {
                animateFrame(deltaTime);
            }
            requestAnimationFrame(gameLoop);
        }
    }

    function reset() {
        simulator.reset();
        curr = simulator.state;
        simulator.step();
        next = simulator.state;
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

    export const setSpeed = (interval: number) => {
        tickInterval = interval;
    };

    export const setWeights = (weights: Weights) => {
        simulator.update_weights(weights);
    };

    onMount(() => {
        reset();
    });
</script>

<div>
    <div class="tetris-controls">
        <button onclick={() => reset()} disabled={isRunning}>Reset</button>
        <button onclick={() => togglePaused()} disabled={gameOver}
            >{isRunning ? "Pause" : "Play"}</button
        >
        <button onclick={() => step()} disabled={isRunning}>Step</button>
        <span>Moves/second: {displayMoves}</span>
    </div>
    <TetrisBoard bind:this={tetrisBoard} />
</div>

<style>
    .tetris-controls {
        justify-content: center;
        display: flex;
        gap: 8px;
        margin-bottom: 8px;
    }
</style>
