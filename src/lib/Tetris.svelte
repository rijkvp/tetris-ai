<script lang="ts">
    import { t } from "$lib/translations";
    import { onDestroy } from "svelte";
    import { Simulator, WeightsMap, Path } from "tetris-ai";
    import type { GameState, Stats } from "$lib/types.ts";
    import TetrisBoard from "$lib/TetrisBoard.svelte";
    import StatsPanel from "$lib/StatsPanel.svelte";
    import GameControls from "$lib/GameControls.svelte";
    import { onMount } from "svelte";

    const SPEED_MUTIPLIER = [
        0.2, 0.5, 1, 2, 5, 10, 20, 50, 100, 1000, 10000, 100000, 1000000,
    ];

    let {
        onNewStats,
        onGameOver,
        maxSpeed = SPEED_MUTIPLIER.length - 1,
    }: {
        onNewStats?: (stats: Stats) => void;
        onGameOver: (stats: Stats) => void;
        maxSpeed?: number;
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
    const BASE_SPEED = 1 / 5; // base interval between ticks in seconds
    let tickInterval = BASE_SPEED;

    let speedIndex = $state(2);
    let speedMultiplier = $derived(SPEED_MUTIPLIER[speedIndex]);

    let moves = 0;
    let lastMoves = 0;

    const moveTimer = setInterval(() => {
        statsPanel.setMoves(moves - lastMoves);
        lastMoves = moves;
    }, 1000);

    let animationFrame: number;

    function simulateNext(): boolean {
        curr = next;
        if (onNewStats) onNewStats(curr.stats);
        if (!simulator.step()) {
            curr = simulator.state;
            // game over
            gameOver = true;
            if (onGameOver) onGameOver(curr.stats);
            isRunning = false;
            return false;
        }
        moves++;
        next = simulator.state;
        path = simulator.path!;
        return true;
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

    // based on NES Tetris
    function framesPerDrop(level: number): number {
        if (level < 0) return NaN;
        if (level <= 8) return 48 - 5 * level;
        if (level <= 18) return 6 - Math.floor((level - 7) / 3);
        if (level <= 28) return 2;
        return 1;
    }

    function gameLoop(currentTime: number) {
        const weightedSpeed =
            (speedMultiplier / framesPerDrop(Number(curr.stats.level) + 1)) *
            10;
        tickInterval = (1 / weightedSpeed) * BASE_SPEED;

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
            animationFrame = requestAnimationFrame(gameLoop);
        }
    }

    function newGame() {
        simulator.reset();
        curr = simulator.state;
        simulator.step();
        next = simulator.state;
        path = simulator.path!;

        tetrisBoard.clear();

        tick = 0;
        tickTimer = 0;

        gameOver = false;
        isRunning = true;
        // start the game loop / animation
        lastFrameTime = performance.now(); // prevents time from 'ticking' while paused
        animationFrame = requestAnimationFrame(gameLoop);
    }

    function togglePaused() {
        isRunning = !isRunning;
        if (isRunning) {
            // start the game loop / animation
            lastFrameTime = performance.now(); // prevents time from 'ticking' while paused
            animationFrame = requestAnimationFrame(gameLoop);
        }
    }

    export const setWeights = (weights: [string, number][]) => {
        simulator.update_weights(WeightsMap.from_js(weights));
    };

    onMount(() => {
        newGame();
    });
    onDestroy(() => {
        cancelAnimationFrame(animationFrame);
        clearInterval(moveTimer);
    });
</script>

<div class="grid">
    <div class="stats">
        <StatsPanel bind:this={statsPanel} />
    </div>
    <div class="controls">
        <GameControls
            bind:isRunning
            bind:gameOver
            onPauseToggle={() => togglePaused()}
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
        <TetrisBoard bind:this={tetrisBoard} bind:statsPanel />
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
