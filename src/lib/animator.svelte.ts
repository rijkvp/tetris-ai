import type { Move } from "tetris-ai";
import type { TetrisSimulator } from "./simulator.svelte";

const BASE_SPEED = 1 / 5; // base interval between ticks in seconds

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
    #tickInterval = BASE_SPEED;
    #accumulator = 0;
    #animationFrame: number = -1;
    #lastFrameTime = 0;

    constructor(simulator: TetrisSimulator, onDisplay: () => void, onGameOver: () => void) {
        this.#simulator = simulator;
        this.#onDisplay = onDisplay;
        this.#onGameOver = onGameOver;
    }

    #gameOver() {
        this.isRunning = false;
        this.#onGameOver();
    }

    #gameLoop(currentTime: number) {
        const weightedSpeed =
            (this.#speedMultiplier /
                framesPerDrop(Number(this.#simulator.state.stats.level) + 1)) *
            10;
        this.#tickInterval = (1 / weightedSpeed) * BASE_SPEED;

        const deltaTime = (currentTime - this.#lastFrameTime) / 1000;
        this.#lastFrameTime = currentTime;
        this.#accumulator += deltaTime;

        if (this.isRunning) {
            // Simulate whole moves based on accumulated time
            while (this.#accumulator >= this.#tickInterval * this.#simulator.pathLength) {
                this.#accumulator -= this.#tickInterval * this.#simulator.pathLength;
                if (!this.#simulator.simulateNext()) {
                    this.#gameOver();
                    break;
                }
            }

            const progress = this.#accumulator / (this.#tickInterval * this.#simulator.pathLength);
            this.#animateFrame(Math.min(Math.max(progress, 0), 0.999)); // animate the current frame

            this.#animationFrame = requestAnimationFrame(this.#gameLoop.bind(this));
        }
    }

    #animateFrame(progress: number) {
        const tick = progress * this.#simulator.pathLength;
        const tickIndex = Math.floor(tick);
        const tickProgress = tick - tickIndex;
        this.currentMove = this.#simulator.path?.transition_move(tickIndex, tickProgress) ?? null;
        this.#onDisplay();
    }


    setSpeed(speed: number) {
        this.#speedMultiplier = speed;
        // Fix accumulator to prevent jumps in time
        const weightedSpeed =
            (this.#speedMultiplier /
                framesPerDrop(Number(this.#simulator.state.stats.level) + 1)) *
            10;
        const newTickInterval = (1 / weightedSpeed) * BASE_SPEED;
        this.#accumulator = this.#accumulator * (newTickInterval / this.#tickInterval);
        this.#tickInterval = newTickInterval;
    }

    restart() {
        this.isRunning = true;
        this.#accumulator = 0;
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
