<script lang="ts">
    import { t } from "$lib/translations";
    import { theme } from "$lib/theme.svelte";
    import { onDestroy } from "svelte";
    import { Simulator, WeightsMap, Path } from "tetris-ai";
    import type { GameState, Stats } from "$lib/types.ts";
    import TetrisBoard from "$lib/TetrisBoard.svelte";
    import StatsPanel from "$lib/StatsPanel.svelte";
    import { onMount } from "svelte";
    import { base } from "$app/paths";

    const SPEED_MUTIPLIER = [
        0.1, 0.5, 1, 2, 5, 10, 20, 50, 100, 1000, 10000, 100000, 1000000,
    ];

    let {
        onNewStats,
        onGameOver,
        maxSpeed = SPEED_MUTIPLIER.length - 1,
    }: {
        onNewStats: (stats: Stats) => void;
        onGameOver: (stats: Stats) => void;
        maxSpeed: number;
    } = $props();

    let simulator: Simulator = new Simulator();
    let tetrisBoard: TetrisBoard;
    let statsPanel: StatsPanel = $state()!;

    let isRunning = $state(false);
    let gameOver = $state(false);

    // TODO: Cleanup this mess
    let curr: GameState, next: GameState;
    let path: Path;

    let lastFrameTime = 0;
    let tick = 0;
    let tickTimer = 0;
    const BASE_SPEED = 1 / 8; // base interval between ticks in seconds
    let tickInterval = BASE_SPEED;

    let speedIndex = $state(2);
    let speedMultiplier = $state(1);

    let moves = 0;
    let lastMoves = 0;

    const moveTimer = setInterval(() => {
        statsPanel.setMoves(moves - lastMoves);
        lastMoves = moves;
    }, 1000);

    onDestroy(() => {
        clearInterval(moveTimer);
    });

    function simulateNext(): boolean {
        curr = next;
        onNewStats(curr.stats);
        if (!simulator.step()) {
            curr = simulator.state;
            next = null;
            // game over
            gameOver = true;
            onGameOver(curr.stats);
            console.log("Game over");
            console.log(curr);
            console.log(next);
            isRunning = false;
            return false;
        }
        moves++;
        next = simulator.state;
        path = simulator.path;
        return true;
    }

    function step() {
        simulateNext();
        tetrisBoard.display(curr, null);
    }

    function animateFrame(deltaTime: number) {
        // update tick timers, multiple ticks can occur in a single frame
        tickTimer += deltaTime;
        while (tickTimer >= tickInterval) {
            tickTimer -= tickInterval;
            tick++;
        }
        // check if move is complete
        if (tick >= path.length) {
            simulateNext();
            tick = 0;
            tickTimer = 0;
        }
        const tickProgress = tickTimer / tickInterval; // progress from 0 to 1 within a tick
        const currentMove = path.transition_move(tick, tickProgress);
        tetrisBoard.display(curr, currentMove);
    }

    const targetFPS = 60; // TODO: measure exact time spent on rendering
    const maxFrameDuration = 1 / targetFPS; // maximum time to spend on rendering a frame

    function gameLoop(currentTime: number) {
        // ensure deltaTime is at least 1ms, to avoid division by zero or negative values
        const deltaTime = Math.max(currentTime - lastFrameTime, 1) / 1000;
        lastFrameTime = currentTime;
        // console.log(`Delta time: ${deltaTime}, FPS: ${1 / deltaTime}`);
        if (isRunning) {
            // if the complete animation of the move takes less than the frame duration
            if (tickInterval * path.length < deltaTime) {
                const ticksGoal = Math.floor(maxFrameDuration / tickInterval); // ideally, we want to complete about many ticks to reach the ticks/second goal
                let ticksSpent = path.length;
                while (ticksSpent < ticksGoal) {
                    if (!simulateNext()) {
                        break;
                    }
                    ticksSpent += path.length;
                    // if current update is taking longer than the max frame duration, break
                    if (
                        performance.now() - currentTime >
                        maxFrameDuration * 1000
                    ) {
                        break;
                    }
                }
                // console.log(`${ticksSpent} ticks spent of ${ticksGoal}`);
                if (ticksSpent >= ticksGoal) {
                    console.warn("Exceeded ticks goal");
                }
                tetrisBoard.display(curr, null); // finally display the final state
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
        path = simulator.path;
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

    function updateSpeed() {
        speedMultiplier = SPEED_MUTIPLIER[speedIndex];
        tickInterval = (1 / speedMultiplier) * BASE_SPEED;
    }

    export const setWeights = (weights: [string, number][]) => {
        simulator.update_weights(WeightsMap.from_js(weights));
    };

    onMount(() => {
        reset();
    });

    let modeName2 = $derived(() => (theme.prefersDark ? "dark" : "light"));
</script>

<div class="grid">
    <div class="stats">
        <StatsPanel bind:this={statsPanel} />
    </div>
    <div class="controls">
        <div>
            <input
                class="speed-input"
                title={$t("speed")}
                type="range"
                min="0"
                max={Math.min(SPEED_MUTIPLIER.length - 1, maxSpeed)}
                bind:value={speedIndex}
                oninput={() => updateSpeed()}
            />
            <span class="speed-display"
                >{speedMultiplier.toLocaleString()}x</span
            >
        </div>
        <div>
            <button
                onclick={() => reset()}
                disabled={isRunning}
                title={$t("controls.reset")}
            >
                <img src="{base}/icons/reset.png" alt="Reset" />
                {$t("controls.reset")}
            </button>
            <button
                onclick={() => togglePaused()}
                disabled={gameOver}
                title={isRunning ? $t("controls.pause") : $t("controls.play")}
            >
                {#if isRunning}
                    {$t("controls.pause")}
                {:else}
                    <img
                        src={`${base}/icons/play-${theme.modeName}.png`}
                        alt="Play"
                    />
                    {$t("controls.play")}
                {/if}
            </button>
            <button
                onclick={() => step()}
                disabled={isRunning}
                title={$t("controls.step")}
            >
                <img src="{base}/icons/skip.png" alt="Step" />
                {$t("controls.step")}
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
        /* gap: 8px; */
        /* margin-bottom: 8px; */
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
    .controls button {
        width: 4rem;
        height: 1.5rem;
        font-size: 1.2rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    img {
        image-rendering: optimizeSpeed;
        image-rendering: -o-crisp-edges;
        image-rendering: -webkit-optimize-contrast;
        image-rendering: -moz-crisp-edges;
        image-rendering: crisp-edges;
        image-rendering: pixelated;
    }
</style>
