import { Simulator, type Path, type WeightsMap } from "tetris-ai";
import type { TetrisState } from "./types";

export class TetrisSimulator {
    #simulator: Simulator = new Simulator();
    state: TetrisState = $state({
        board: this.#simulator.state.board,
        stats: this.#simulator.state.stats,
        gameOver: false,
    });
    #next: TetrisState = this.#simulator.state;
    #path: Path | null = null;

    constructor() {
        this.reset();
    }

    reset() {
        this.#simulator.reset();
        this.state = this.#simulator.state;
        this.state.gameOver = false;
        this.#simulator.step();
        this.#next = this.#simulator.state;
        this.#path = this.#simulator.path ?? null;
    }

    updateWeights(weightsMap: WeightsMap) {
        this.#simulator.update_weights(weightsMap);
    }

    simulateNext(): boolean {
        this.state = this.#next;
        if (this.state.gameOver) {
            // game over
            return false;
        }
        this.#simulator.step();
        this.#next = this.#simulator.state;
        this.#path = this.#simulator.path ?? null;
        return true;
    }

    get path(): Path | null {
        return this.#path;
    }

    get pathLength() {
        if (!this.path) return 0;
        return this.path.length;
    }
}
