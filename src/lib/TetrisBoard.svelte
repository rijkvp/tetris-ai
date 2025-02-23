<script lang="ts">
    import { onMount } from "svelte";
    import { get_piece_rotation } from "tetris-ai";
    import type { GameState, Move } from "$lib/types.ts";
    import StatsPanel from "$lib/StatsPanel.svelte";

    const BOARD_WIDTH = 10;
    const BOARD_HEIGHT = 20;
    const OUTLINE = 2;

    const CELL_SIZE = 32;
    const CELL_COLOURS = [
        [180, 100, 47], // #00f0f0
        [280, 100, 47], // #a000f0
        [40, 100, 47], // #f0a000
        [240, 100, 47], // #0000f0
        [60, 100, 47], // #f0f000
        [0, 100, 47], // #f00000
        [120, 100, 47], // #00f000
    ];

    let canvas: HTMLCanvasElement;
    let statsPanel: StatsPanel;
    let context: CanvasRenderingContext2D;

    function displayCell(col: number, row: number, pieceIdx: number) {
        const color = CELL_COLOURS[pieceIdx];
        context.fillStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2]}%)`;
        // context.strokeStyle = "none";
        const px = col * CELL_SIZE;
        const py = row * CELL_SIZE;
        context.fillRect(px, py, CELL_SIZE, CELL_SIZE);

        context.lineWidth = OUTLINE;
        // left
        context.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 8}%)`;
        context.beginPath();
        context.moveTo(px + OUTLINE / 2, py);
        context.lineTo(px + OUTLINE / 2, py + CELL_SIZE);
        context.stroke();
        // top
        context.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] + 20}%)`;
        context.beginPath();
        context.moveTo(px, py + OUTLINE / 2);
        context.lineTo(px + CELL_SIZE, py + OUTLINE / 2);
        context.stroke();
        // right
        context.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 15}%)`;
        context.beginPath();
        context.moveTo(px + CELL_SIZE - OUTLINE / 2, py);
        context.lineTo(px + CELL_SIZE - OUTLINE / 2, py + CELL_SIZE);
        context.stroke();
        // bottom
        context.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 20}%)`;
        context.beginPath();
        context.moveTo(px, py + CELL_SIZE - OUTLINE / 2);
        context.lineTo(px + CELL_SIZE, py + CELL_SIZE - OUTLINE / 2);
        context.stroke();
    }

    function displayCellOutline(col: number, row: number) {
        context.strokeStyle = "#f7fc76";
        context.lineWidth = OUTLINE;
        const px = col * CELL_SIZE;
        const py = row * CELL_SIZE;
        context.strokeRect(
            px + OUTLINE / 2,
            py + OUTLINE / 2,
            CELL_SIZE,
            CELL_SIZE,
        );
    }

    function displayBoard(board: Uint8Array[]) {
        for (let row = 0; row < BOARD_HEIGHT; row++) {
            for (let col = 0; col < BOARD_WIDTH; col++) {
                const cellValue = board[row][col];
                if (cellValue === 0) {
                    continue;
                }
                displayCell(col, row, cellValue - 1);
            }
        }
    }

    function displayPiece(pieceIdx: number, move: Move) {
        const pattern = get_piece_rotation(pieceIdx, move.rot);

        for (let r = 0; r < pattern.size; r++) {
            for (let c = 0; c < pattern.size; c++) {
                const idx = r * pattern.size + c;
                if (!pattern.data[idx]) {
                    continue;
                }
                displayCell(move.col + c, move.row + r, pieceIdx);
            }
        }
    }

    function displayOutline(pieceIdx: number, move: Move) {
        const pattern = get_piece_rotation(pieceIdx, move.rot);
        for (let r = 0; r < pattern.size; r++) {
            for (let c = 0; c < pattern.size; c++) {
                const idx = r * pattern.size + c;
                if (!pattern.data[idx]) {
                    continue;
                }
                displayCellOutline(move.col + c, move.row + r);
            }
        }
    }

    export const clear = () => {
        context.clearRect(0, 0, canvas.width, canvas.height);
        context.lineWidth = 1;
        context.strokeStyle = "#333";
        // draw lines
        const width = BOARD_WIDTH * CELL_SIZE;
        const height = BOARD_HEIGHT * CELL_SIZE;
        for (let r = 0; r <= BOARD_HEIGHT; r++) {
            const y = r * CELL_SIZE;
            context.beginPath();
            context.moveTo(0, y + 0.5);
            context.lineTo(width, y + 0.5);
            context.stroke();
        }
        for (let c = 0; c <= BOARD_WIDTH; c++) {
            const x = c * CELL_SIZE;
            context.beginPath();
            context.moveTo(x + 0.5, 0);
            context.lineTo(x + 0.5, height);
            context.stroke();
        }
    };

    export const display = (state: GameState) => {
        clear();
        displayBoard(state.board);
        statsPanel.update(state.stats);
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
            const currentPath = currentMove.path[tick];
            const currentPathIndex = Math.min(
                Math.floor(tickProgress * currentPath.length),
                currentPath.length - 1,
            );
            displayPiece(currentMove.piece_idx, currentPath[currentPathIndex]);
            // Uncomment to display outline of target position
            // const lastPath = currentMove.path[next.move.path.length - 1];
            // const lastMove = lastPath[lastPath.length - 1];
            // displayOutline(currentMove.piece_idx, lastMove);
        }

        statsPanel.update(state.stats);
    };

    onMount(() => {
        context = canvas.getContext("2d")!;
    });
</script>

<div class="board">
    <StatsPanel bind:this={statsPanel} />
    <canvas bind:this={canvas} width="320" height="640"></canvas>
</div>

<style>
    .board {
        display: flex;
        gap: 16px;
        align-items: flex-start;
    }
    canvas {
        border: 2px solid #fff;
        border-radius: 0px;
        background-color: #070707;
        box-shadow:
            0 4px 8px 0 rgba(0, 0, 0, 0.2),
            0 6px 20px 0 rgba(0, 0, 0, 0.19);
    }
</style>
