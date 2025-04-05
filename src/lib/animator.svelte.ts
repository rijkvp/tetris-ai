import type { Move } from "tetris-ai";
import type { TetrisSimulator } from "./simulator.svelte";

const BASE_SPEED = 1 / 5; // base interval between ticks in seconds
const MAX_FRAME_DURATION = 1 / 60; // maximum time to spend on rendering a frame

// based on NES Tetris
function framesPerDrop(level: number): number {
    if (level < 0) return NaN;
    if (level <= 8) return 48 - 5 * level;
    if (level <= 18) return 6 - Math.floor((level - 7) / 3);
    if (level <= 28) return 2;
    return 1;
}

export class TetrisAnimator {
    currentMove: Move | null = $state(null);
    isRunning: boolean = $state(false);

    #simulator: TetrisSimulator;
    #onDisplay: () => void;
    #onGameOver: () => void;

    #speedMultiplier: number = 1;
    #lastFrameTime = 0;
    #tick = 0;
    #tickTimer = 0;
    #tickInterval = BASE_SPEED;
    #animationFrame: number = -1;

    constructor(simulator: TetrisSimulator, onDisplay: () => void, onGameOver: () => void) {
        this.#simulator = simulator;
        this.#onDisplay = onDisplay;
        this.#onGameOver = onGameOver;
    }

    #gameOver() {
        this.isRunning = false;
        this.#onGameOver();
    }

    #animateFrame(deltaTime: number) {
        // update tick timers, multiple ticks can occur in a single frame
        this.#tickTimer += deltaTime;

        while (this.#tickTimer >= this.#tickInterval) {
            this.#tickTimer -= this.#tickInterval;
            this.#tick++;
        }
        // check if move is complete
        if (this.#tick >= this.#simulator.pathLength) {
            if (!this.#simulator.simulateNext()) {
                this.#gameOver();
            }
            this.#tick = 0;
            this.#tickTimer = 0;
        }
        const tickProgress = this.#tickTimer / this.#tickInterval; // progress from 0 to 1 within a tick
        this.currentMove = this.#simulator.path?.transition_move(this.#tick, tickProgress) ?? null;
        this.#onDisplay();
    }

    #gameLoop(currentTime: number) {
        const weightedSpeed =
            (this.#speedMultiplier /
                framesPerDrop(Number(this.#simulator.state.stats.level) + 1)) *
            10;
        this.#tickInterval = (1 / weightedSpeed) * BASE_SPEED;

        // ensure deltaTime is at least 1ms, to avoid division by zero or negative values
        const deltaTime = Math.max(currentTime - this.#lastFrameTime, 1) / 1000;
        this.#lastFrameTime = currentTime;
        // console.log(`Delta time: ${deltaTime}, FPS: ${1 / deltaTime}`);
        if (this.isRunning) {
            this.currentMove = null; // no animated move
            // if the complete animation of the move takes less than the frame duration
            if (this.#tickInterval * this.#simulator.pathLength < deltaTime) {
                const ticksGoal = Math.floor(
                    MAX_FRAME_DURATION / this.#tickInterval,
                ); // ideally, we want to complete about many ticks to reach the ticks/second goal
                let ticksSpent = this.#simulator.pathLength;
                while (ticksSpent < ticksGoal) {
                    if (!this.#simulator.simulateNext()) {
                        this.#gameOver();
                        break;
                    }
                    ticksSpent += this.#simulator.pathLength;
                    // if current update is taking longer than the max frame duration, break
                    if (
                        performance.now() - currentTime >
                        MAX_FRAME_DURATION * 1000
                    ) {
                        break;
                    }
                }
                // console.log(`${ticksSpent} ticks spent of ${ticksGoal}`);
                this.#onDisplay(); // finally display the final state
            } else {
                this.#animateFrame(deltaTime);
            }
            this.#animationFrame = requestAnimationFrame(this.#gameLoop.bind(this));
        }
    }

    setSpeed(speed: number) {
        this.#speedMultiplier = speed;
    }

    restart() {
        this.#tick = 0;
        this.#tickTimer = 0;
        this.isRunning = true;
        // start the game loop / animation
        this.#lastFrameTime = performance.now(); // prevents time from 'ticking' while paused
        this.#animationFrame = requestAnimationFrame(this.#gameLoop.bind(this));
    }

    togglePaused() {
        this.isRunning = !this.isRunning;
        if (this.isRunning) {
            // start the game loop / animation
            this.#lastFrameTime = performance.now(); // prevents time from 'ticking' while paused
            this.#animationFrame = requestAnimationFrame(this.#gameLoop.bind(this));
        }
    }

    stop() {
        cancelAnimationFrame(this.#animationFrame);
    }
}
