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
    let speedMultiplier = 1;
    const BASE_SPEED = 1 / 8; // base interval between ticks in seconds
    let tickInterval = BASE_SPEED;

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
        // update tick timers, multiple ticks can occur in a single frame
        tickTimer += deltaTime;
        while (tickTimer >= tickInterval) {
            tickTimer -= tickInterval;
            tick++;
        }
        // check if move is complete
        if (tick >= next.move.path.length) {
            calcNext();
            tick = 0;
            tickTimer = 0;
        }
        const tickProgress = tickTimer / tickInterval; // progress from 0 to 1 within a tick
        tetrisBoard.displayTransition(curr, next, tick, tickProgress);
    }

    const targetFPS = 60; // TODO: measure exact time spent on rendering
    const maxFrameDuration = 1 / targetFPS / 2; // divide by 2 to allow time for drawing

    function gameLoop(currentTime: number) {
        // ensure deltaTime is at least 1ms, to avoid division by zero or negative values
        const deltaTime = Math.max(currentTime - lastFrameTime, 1) / 1000;
        lastFrameTime = currentTime;
        console.log(`Delta time: ${deltaTime}, FPS: ${1 / deltaTime}`);
        if (isRunning) {
            // if the complete animation of the move takes less than the frame duration
            if (tickInterval * next.move.path.length < deltaTime) {
                const ticksGoal = Math.floor(maxFrameDuration / tickInterval); // ideally, we want to complete about many ticks to reach the ticks/second goal
                let ticksSpent = next.move.path.length;
                while (ticksSpent < ticksGoal) {
                    if (!calcNext()) {
                        break;
                    }
                    ticksSpent += next.move.path.length;
                    // if current update is taking longer than the max frame duration, break
                    if (
                        performance.now() - currentTime >
                        maxFrameDuration * 1000
                    ) {
                        break;
                    }
                }
                console.log(`${ticksSpent} ticks spent of ${ticksGoal}`);
                if (ticksSpent >= ticksGoal) {
                    console.warn("Exceeded ticks goal");
                }
                tetrisBoard.display(curr); // finally display the final state
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

    export const setSpeed = (multiplier: number) => {
        speedMultiplier = multiplier;
        tickInterval = (1 / speedMultiplier) * BASE_SPEED;
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
