<script lang="ts">
    import { Simulator, Weights } from "tetris-ai";
    import type { GameState } from "$lib/types.ts";
    import TetrisBoard from "$lib/TetrisBoard.svelte";
    import { onMount } from "svelte";

    let simulator: Simulator = new Simulator();

    let tetrisBoard: TetrisBoard;
    let isRunning = false;
    let animate = true;

    let state: GameState, next: GameState;

    let lastFrameTime = 0;
    let tick = 0;
    let tickTimer = 0;
    let tickInterval = 1 / 8;

    function calcNext() {
        state = next;
        if (!simulator.step()) {
            // game over
            isRunning = false;
        }
        next = simulator.state;
    }

    function step() {
        calcNext();
        tetrisBoard.display(state);
    }

    function animateFrame(currentTime: number) {
        // check if frame is over
        if (tick > next.move.path.length) {
            calcNext();
            tick = 0;
            tickTimer = 0;
            lastFrameTime = currentTime;
        }

        // update tick timers
        // ensure deltaTime is at least 1ms, to avoid division by zero or negative values
        const deltaTime = Math.max(currentTime - lastFrameTime, 1) / 1000;
        lastFrameTime = currentTime;
        tickTimer += deltaTime;
        while (tickTimer >= tickInterval) {
            tickTimer -= tickInterval;
            tick++;
        }
        const tickProgress = tickTimer / tickInterval; // progress from 0 to 1 within a tick

        tetrisBoard.displayTransition(state, next, tick, tickProgress);
    }

    function gameLoop(currentTime: number) {
        if (isRunning) {
            if (!animate) {
                step();
            } else {
                animateFrame(currentTime);
            }
            requestAnimationFrame(gameLoop);
        }
    }

    function reset() {
        simulator = new Simulator();
        state = simulator.state;
        simulator.step();
        next = simulator.state;
        tetrisBoard.clear();
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

<div class="tetris">
    <TetrisBoard bind:this={tetrisBoard} />
    <div class="tetris-controls">
        <button on:click={() => togglePaused()}
            >{isRunning ? "Pause" : "Play"}</button
        >
        <button on:click={() => step()} disabled={isRunning}>Step</button>
        <button on:click={() => reset()} disabled={isRunning}>Reset</button>
        <input id="enable-animation" type="checkbox" bind:checked={animate} />
        <label for="enable-animation">Animate</label>
    </div>
</div>
