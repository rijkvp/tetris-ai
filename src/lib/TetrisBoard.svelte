<script lang="ts">
    import { onMount } from "svelte";
    import { get_piece_rotation } from "tetris-ai";
    import type { GameState, Stats, Move } from "$lib/types.ts";

    const BOARD_WIDTH = 10;
    const BOARD_HEIGHT = 20;

    const CELL_SIZE = 32;
    const CELL_COLOURS = [
        "#00f0f0",
        "#a000f0",
        "#f0a000",
        "#0000f0",
        "#f0f000",
        "#f00000",
        "#00f000",
    ];

    function displayBoard(board: Uint8Array[]) {
        for (let row = 0; row < BOARD_HEIGHT; row++) {
            for (let col = 0; col < BOARD_WIDTH; col++) {
                const cellValue = board[row][col];
                if (cellValue === 0) {
                    continue;
                }
                context.fillStyle = CELL_COLOURS[cellValue - 1];
                const x = col * CELL_SIZE;
                const y = row * CELL_SIZE;
                context.fillRect(x, y, CELL_SIZE, CELL_SIZE);
                context.lineWidth = 2;
                context.strokeRect(x, y, CELL_SIZE, CELL_SIZE);
            }
        }
    }

    function displayPiece(pieceIdx: number, move: Move) {
        const rot = move.rot;
        const col = move.col;
        const row = move.row;
        const pattern = get_piece_rotation(pieceIdx, rot);

        for (let r = 0; r < pattern.size; r++) {
            for (let c = 0; c < pattern.size; c++) {
                const idx = r * pattern.size + c;
                if (!pattern.data[idx]) {
                    continue;
                }
                context.fillStyle = CELL_COLOURS[pieceIdx];
                const px = (col + c) * CELL_SIZE;
                const py = (row + r) * CELL_SIZE;
                context.fillRect(px, py, CELL_SIZE, CELL_SIZE);
                context.lineWidth = 2;
                context.strokeRect(px, py, CELL_SIZE, CELL_SIZE);
            }
        }
    }

    function displayHud(stats: Stats) {
        context.fillStyle = "#000";
        context.font = "16px monospace";
        context.fillText(`Score: ${stats.score}`, 10, 20);
        context.fillText(`Lines: ${stats.lines}`, 10, 40);
        context.fillText(`Level: ${stats.level}`, 10, 60);
    }

    let canvas: HTMLCanvasElement;
    let context: CanvasRenderingContext2D;

    export const clear = () => {
        context.clearRect(0, 0, canvas.width, canvas.height);
    };

    export const display = (state: GameState) => {
        clear();
        displayBoard(state.board);
        displayHud(state.stats);
    };

    export const displayTransition = (
        state: GameState,
        next: GameState,
        tick: number,
        tickProgress: number,
    ) => {
        clear();
        displayBoard(state.board);

        const currentMove = next.move;
        if (currentMove != null) {
            const tickIndex = Math.min(tick, currentMove.path.length - 1);
            const tickPath = currentMove.path[tickIndex];
            const tickPathIdx = Math.min(
                Math.floor(tickProgress * tickPath.length),
                tickPath.length - 1,
            );
            displayPiece(currentMove.piece_idx, tickPath[tickPathIdx]);
        }

        displayHud(state.stats);
    };

    onMount(() => {
        context = canvas.getContext("2d")!;
    });
</script>

<canvas bind:this={canvas} width="320" height="640"></canvas>

<style>
    canvas {
        border: 2px solid #444;
        background-color: #ddd;
    }
</style>
