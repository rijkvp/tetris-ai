<script lang="ts">
    import { onMount } from "svelte";

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
    let context: CanvasRenderingContext2D;

    function displayCell(col: number, row: number, pieceIdx: number) {
        const color = CELL_COLOURS[pieceIdx];
        context.fillStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2]}%)`;
        // context.strokeStyle = "none";
        const x = col * CELL_SIZE;
        const y = row * CELL_SIZE;
        context.fillRect(x, y, CELL_SIZE, CELL_SIZE);

        context.lineWidth = OUTLINE;
        // left
        context.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 8}%)`;
        context.beginPath();
        context.moveTo(x + OUTLINE / 2, y);
        context.lineTo(x + OUTLINE / 2, y + CELL_SIZE);
        context.stroke();
        // top
        context.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] + 20}%)`;
        context.beginPath();
        context.moveTo(x, y + OUTLINE / 2);
        context.lineTo(x + CELL_SIZE, y + OUTLINE / 2);
        context.stroke();
        // right
        context.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 15}%)`;
        context.beginPath();
        context.moveTo(x + CELL_SIZE - OUTLINE / 2, y);
        context.lineTo(x + CELL_SIZE - OUTLINE / 2, y + CELL_SIZE);
        context.stroke();
        // bottom
        context.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 20}%)`;
        context.beginPath();
        context.moveTo(x, y + CELL_SIZE - OUTLINE / 2);
        context.lineTo(x + CELL_SIZE, y + CELL_SIZE - OUTLINE / 2);
        context.stroke();
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

    function clear() {
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
    }

    function highlightLine(x1: number, y1: number, x2: number, y2: number) {
        context.lineWidth = 5;
        context.strokeStyle = "#f00";
        context.beginPath();
        context.moveTo(x1, y1);
        context.lineTo(x2, y2);
        context.stroke();
    }

    function highlightCell(col: number, row: number) {
        context.strokeStyle = "#f7fc76";
        context.lineWidth = OUTLINE;
        const px = col * CELL_SIZE;
        const py = row * CELL_SIZE;
        context.strokeRect(
            px - OUTLINE / 2,
            py - OUTLINE / 2,
            CELL_SIZE + OUTLINE / 2,
            CELL_SIZE + OUTLINE / 2,
        );
    }

    function displayColTrans(board: Uint8Array[]) {
        for (let c = 0; c < BOARD_WIDTH; c++) {
            for (let r = 0; r < BOARD_HEIGHT - 1; r++) {
                if ((board[r][c] !== 0) != (board[r + 1][c] !== 0)) {
                    const x = c * CELL_SIZE;
                    const y = r * CELL_SIZE;
                    highlightLine(
                        x,
                        y + CELL_SIZE,
                        x + CELL_SIZE,
                        y + CELL_SIZE,
                    );
                }
            }
        }
    }

    function displayRowTrans(board: Uint8Array[]) {
        for (let r = 0; r < BOARD_HEIGHT; r++) {
            for (let c = 0; c < BOARD_WIDTH - 1; c++) {
                if ((board[r][c] !== 0) != (board[r][c + 1] !== 0)) {
                    const x = c * CELL_SIZE;
                    const y = r * CELL_SIZE;
                    highlightLine(
                        x + CELL_SIZE,
                        y,
                        x + CELL_SIZE,
                        y + CELL_SIZE,
                    );
                }
            }
        }
    }

    function displayPits(board: Uint8Array[], height: number[]) {
        context.lineWidth = 3;
        context.strokeStyle = "#f0f";
        for (let c = 0; c < BOARD_WIDTH; c++) {
            for (let r = BOARD_HEIGHT - height[c]; r < BOARD_HEIGHT; r++) {
                if (board[r][c] === 0) {
                    highlightCell(c, r);
                }
            }
        }
    }

    const TRANS_BOARD: number[][] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [5, 5, 6, 6, 0, 0, 0, 0, 0, 0],
        [5, 5, 2, 6, 6, 0, 0, 1, 5, 5],
        [4, 4, 2, 2, 7, 7, 0, 1, 5, 5],
        [4, 1, 2, 7, 7, 5, 5, 1, 3, 0],
        [4, 1, 2, 3, 3, 5, 5, 1, 3, 0],
        [0, 1, 2, 2, 3, 5, 5, 1, 3, 3],
        [0, 1, 2, 6, 3, 5, 5, 1, 5, 5],
        [4, 4, 6, 0, 1, 1, 1, 3, 3, 3],
    ];

    const HOLES_BOARD = [
        [0, 0, 0, 0, 0, 0, 6, 6, 0, 0],
        [0, 0, 2, 0, 4, 0, 6, 0, 4, 4],
        [5, 5, 2, 2, 4, 4, 4, 3, 4, 0],
        [5, 5, 2, 0, 1, 3, 3, 3, 4, 0],
        [0, 4, 4, 4, 1, 4, 4, 4, 3, 3],
        [0, 3, 3, 4, 1, 0, 6, 4, 2, 3],
        [5, 5, 3, 1, 1, 6, 0, 5, 5, 3],
        [4, 4, 0, 1, 1, 6, 6, 5, 5, 1],
        [4, 0, 2, 1, 1, 4, 6, 6, 0, 1],
        [1, 0, 5, 5, 3, 3, 3, 3, 3, 1],
        [1, 1, 5, 5, 3, 7, 0, 3, 3, 3],
        [1, 1, 3, 3, 0, 7, 7, 3, 3, 1],
        [2, 1, 1, 3, 4, 0, 4, 5, 5, 1],
        [2, 2, 1, 0, 4, 2, 4, 4, 4, 1],
        [2, 0, 1, 2, 2, 2, 2, 2, 3, 3],
        [1, 1, 1, 0, 6, 6, 6, 5, 5, 6],
        [1, 2, 2, 6, 5, 5, 4, 1, 6, 0],
        [1, 2, 7, 0, 5, 5, 4, 1, 2, 0],
        [6, 6, 7, 7, 0, 5, 5, 1, 2, 2],
        [0, 6, 6, 7, 0, 5, 5, 1, 2, 0],
    ];

    const LANDING_HEIGHT_BOARD = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 2, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 5, 5, 0, 0, 0, 0, 0, 2],
        [1, 1, 5, 5, 7, 0, 0, 0, 2, 2],
        [1, 1, 7, 7, 7, 7, 0, 4, 4, 2],
        [1, 7, 7, 7, 7, 7, 0, 4, 5, 5],
        [5, 5, 0, 6, 6, 4, 1, 1, 1, 1],
        [5, 5, 4, 6, 6, 6, 6, 0, 1, 4],
        [3, 6, 6, 2, 7, 7, 0, 3, 1, 1],
    ];

    const ERODED_BOARD = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
        [2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 2, 0, 0, 0, 0, 0, 0, 0, 6],
        [2, 1, 0, 0, 0, 0, 0, 0, 6, 6],
        [1, 1, 5, 5, 0, 0, 0, 0, 6, 2],
        [1, 1, 5, 5, 7, 0, 0, 0, 2, 2],
        [1, 1, 7, 7, 7, 7, 0, 4, 4, 2],
        [1, 7, 7, 7, 7, 7, 0, 4, 5, 5],
        [5, 5, 0, 6, 6, 4, 1, 1, 1, 1],
        [5, 5, 4, 6, 6, 6, 6, 0, 1, 4],
        [3, 6, 6, 2, 7, 7, 0, 3, 1, 1],
    ];


    const WELLS_BOARD = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 6, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 6, 0, 6, 0],
        [0, 0, 0, 0, 0, 6, 2, 6, 6, 0],
        [0, 0, 0, 0, 0, 2, 2, 6, 0, 0],
        [0, 0, 0, 0, 0, 2, 2, 2, 0, 0],
        [0, 3, 3, 3, 0, 2, 2, 2, 2, 0],
        [0, 3, 5, 5, 0, 2, 3, 2, 2, 0],
        [0, 4, 5, 5, 3, 3, 3, 2, 2, 0],
        [0, 4, 5, 5, 5, 5, 6, 6, 2, 0],
        [4, 4, 5, 5, 5, 5, 7, 6, 6, 0],
        [4, 4, 6, 3, 3, 3, 7, 7, 4, 0],
        [4, 6, 6, 3, 3, 3, 3, 7, 4, 0],
        [4, 6, 3, 3, 3, 6, 3, 4, 4, 0],
        [5, 5, 3, 3, 6, 6, 3, 6, 6, 0],
        [1, 6, 6, 3, 5, 5, 3, 7, 7, 0],
        [1, 6, 7, 7, 5, 5, 3, 2, 7, 0],
        [1, 7, 7, 4, 4, 4, 2, 2, 2, 0],
    ];

    function caculateHeight(board: Uint8Array[]): number[] {
        let height: number[] = new Array(BOARD_WIDTH).fill(0);
        for (let c = 0; c < BOARD_WIDTH; c++) {
            for (let r = 0; r < BOARD_HEIGHT; r++) {
                if (board[r][c] !== 0) {
                    height[c] = BOARD_HEIGHT - r;
                    break;
                }
            }
        }
        return height;
    }

    function displayWells(height: number[]) {
        console.log(height);
        context.lineWidth = 3;
        context.strokeStyle = "#f0f";
        const wells = new Array(BOARD_WIDTH).fill(0);
        for (let i = 1; i < BOARD_WIDTH - 1; i++) {
            wells[i] = Math.max(
                0,
                Math.min(height[i - 1], height[i + 1]) - height[i],
            );
        }
        wells[0] = Math.max(0, height[1] - height[0]);
        wells[BOARD_WIDTH - 1] = Math.max(
            0,
            height[BOARD_WIDTH - 2] - height[BOARD_WIDTH - 1],
        );
        console.log(wells);
        for (let c = 0; c < BOARD_WIDTH; c++) {
            for (
                let r = BOARD_HEIGHT - height[c] - wells[c];
                r < BOARD_HEIGHT - height[c];
                r++
            ) {
                highlightCell(c, r);
            }
        }
    }

    function prepareBoard(input: number[][]): [Uint8Array[], number[]] {
        clear();
        const board = input.map((row) => new Uint8Array(row));
        displayBoard(board);
        return [board, caculateHeight(board)];
    }

    export const display = (key: string) => {
        switch (key) {
            case "col_trans":
            case "row_trans":
                const [board, _] = prepareBoard(TRANS_BOARD);
                if (key === "row_trans") {
                    displayRowTrans(board);
                } else {
                    displayColTrans(board);
                }
                break;
            case "pits":
                const [board2, height] = prepareBoard(HOLES_BOARD);
                displayPits(board2, height);
                break;
            case "landing_height":
                prepareBoard(LANDING_HEIGHT_BOARD);
                highlightCell(6, 13);
                highlightCell(6, 14);
                highlightCell(6, 15);
                highlightCell(6, 16);
                highlightLine(
                    0,
                    17 * CELL_SIZE,
                    BOARD_WIDTH * CELL_SIZE,
                    17 * CELL_SIZE,
                );
                break;
            case "eroded_cells":
                prepareBoard(ERODED_BOARD);
                highlightCell(4, 13);
                highlightCell(5, 13);
                highlightCell(6, 13);
                highlightCell(7, 13);
                break;
            case "cuml_wells":
                displayWells(prepareBoard(WELLS_BOARD)[1]);
                break;
            default:
                clear();
        }
    };

    onMount(() => {
        context = canvas.getContext("2d")!;
    });
</script>

<canvas bind:this={canvas} width="320" height="640"></canvas>

<style>
    canvas {
        border: 2px solid #fff;
    }
</style>
