import { prefersDarkMode } from "$lib/theme";

export const BOARD_WIDTH = 10;
export const BOARD_HEIGHT = 20;

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
const CELL_OUTLINE = 3;

const HIGHLIGHT_LINE_COLOUR = "#f00";
const HIGHLIGHT_LINE_WIDTH = 5;

const HIGHLIGHT_CELL_STROKE = "#f7f300";
const HIGHLIGHT_CELL_FILL = "rgba(247, 252, 118, 0.1)"
const HIGHLIGHT_CELL_WIDTH = 4;

export type BoardData = Uint8Array[];

export function displayCell(ctx: CanvasRenderingContext2D, col: number, row: number, pieceIdx: number) {
    const color = CELL_COLOURS[pieceIdx];
    ctx.fillStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2]}%)`;
    const x = col * CELL_SIZE;
    const y = row * CELL_SIZE;
    ctx.fillRect(x, y, CELL_SIZE, CELL_SIZE);

    ctx.lineWidth = CELL_OUTLINE;
    // left
    ctx.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 8}%)`;
    ctx.beginPath();
    ctx.moveTo(x + CELL_OUTLINE / 2, y);
    ctx.lineTo(x + CELL_OUTLINE / 2, y + CELL_SIZE);
    ctx.stroke();
    // top
    ctx.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] + 20}%)`;
    ctx.beginPath();
    ctx.moveTo(x, y + CELL_OUTLINE / 2);
    ctx.lineTo(x + CELL_SIZE, y + CELL_OUTLINE / 2);
    ctx.stroke();
    // right
    ctx.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 15}%)`;
    ctx.beginPath();
    ctx.moveTo(x + CELL_SIZE - CELL_OUTLINE / 2, y);
    ctx.lineTo(x + CELL_SIZE - CELL_OUTLINE / 2, y + CELL_SIZE);
    ctx.stroke();
    // bottom
    ctx.strokeStyle = `hsl(${color[0]}, ${color[1]}%, ${color[2] - 20}%)`;
    ctx.beginPath();
    ctx.moveTo(x, y + CELL_SIZE - CELL_OUTLINE / 2);
    ctx.lineTo(x + CELL_SIZE, y + CELL_SIZE - CELL_OUTLINE / 2);
    ctx.stroke();
}

function displayGrid(ctx: CanvasRenderingContext2D) {
    ctx.lineWidth = 1;
    if (prefersDarkMode) {
        ctx.strokeStyle = "#333";
    } else {
        ctx.strokeStyle = "red";
    }
    // draw lines
    const width = BOARD_WIDTH * CELL_SIZE;
    const height = BOARD_HEIGHT * CELL_SIZE;
    for (let r = 0; r <= BOARD_HEIGHT; r++) {
        const y = r * CELL_SIZE;
        ctx.beginPath();
        ctx.moveTo(0, y + 0.5);
        ctx.lineTo(width, y + 0.5);
        ctx.stroke();
    }
    for (let c = 0; c <= BOARD_WIDTH; c++) {
        const x = c * CELL_SIZE;
        ctx.beginPath();
        ctx.moveTo(x + 0.5, 0);
        ctx.lineTo(x + 0.5, height);
        ctx.stroke();
    }

}

export function highlightLine(ctx: CanvasRenderingContext2D, c1: number, r1: number, c2: number, r2: number) {
    ctx.strokeStyle = HIGHLIGHT_LINE_COLOUR;
    ctx.lineWidth = HIGHLIGHT_LINE_WIDTH;
    ctx.beginPath();
    ctx.moveTo(c1 * CELL_SIZE, r1 * CELL_SIZE);
    ctx.lineTo(c2 * CELL_SIZE, r2 * CELL_SIZE);
    ctx.stroke();
}

export function highlightCell(ctx: CanvasRenderingContext2D, col: number, row: number) {
    ctx.fillStyle = HIGHLIGHT_CELL_FILL;
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx.strokeStyle = HIGHLIGHT_CELL_STROKE;
    ctx.lineWidth = HIGHLIGHT_CELL_WIDTH;
    ctx.strokeRect(
        col * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE,
    );
}

export function clearBoard(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    displayGrid(ctx);
}

export function displayBoard(ctx: CanvasRenderingContext2D, board: BoardData) {
    for (let row = 0; row < BOARD_HEIGHT; row++) {
        for (let col = 0; col < BOARD_WIDTH; col++) {
            const cellValue = board[row][col];
            if (cellValue === 0) {
                continue;
            }
            displayCell(ctx, col, row, cellValue - 1);
        }
    }
}

