import init, { Simulator, get_piece_rotation } from "./pkg/tetris_ai.js";

await init();

const BOARD_WIDTH = 10;
const BOARD_HEIGHT = 20;

const CELL_SIZE = 32;
const CELL_COLOURS = [
    '#00f0f0',
    '#a000f0',
    '#f0a000',
    '#0000f0',
    '#f0f000',
    '#f00000',
    '#00f000',
];

let tickRate = 10;
let tick = 0;

const canvas = document.getElementById('game');
const context = canvas.getContext('2d');

const speedSlider = document.getElementById('speed-slider');
const speedLabel = document.getElementById('speed-label');
const stepLabel = document.getElementById('step-label');
speedSlider.addEventListener('input', () => {
    tickRate = speedSlider.value;
    speedLabel.innerText = `${tickRate} t/s`;
});

const simulator = new Simulator();
let currentMove = null;
let board = null;
let nextBoard = null;

function drawBoard(board) {
    for (let row = 0; row < BOARD_HEIGHT; row++) {
        for (let col = 0; col < BOARD_WIDTH; col++) {
            const cellValue = board[row * BOARD_WIDTH + col];
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

function drawOutline(pieceIdx, move) {
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
            context.fillStyle = '#aaa';
            const px = (col + c) * CELL_SIZE;
            const py = (row + r) * CELL_SIZE;
            context.fillRect(px, py, CELL_SIZE, CELL_SIZE);
        }
    }

}

function drawPiece(pieceIdx, move) {
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


function draw() {
    // clear
    context.clearRect(0, 0, canvas.width, canvas.height);

    // UI
    stepLabel.innerText = `Placed pieces: ${simulator.steps}`;

    // draw move
    if (currentMove != null) {
        // currentMove.path.forEach((move) => {
        //     drawOutline(currentMove.piece_idx, move);
        // });
        const moveProgress = tick;
        drawPiece(currentMove.piece_idx, currentMove.path[Math.min(moveProgress, currentMove.path.length - 1)]);
    }
    // draw the board
    if (board !== null)
        drawBoard(board);
}

let lastFrameTime = 0;
let tickAccumulator = 0;

function gameLoop(currentTime) {
    const deltaTime = (currentTime - lastFrameTime) / 1000;
    lastFrameTime = currentTime;
    tickAccumulator += deltaTime;

    const tickInterval = 1 / tickRate;
    while (tickAccumulator >= tickInterval) {
        tickAccumulator -= tickInterval;
        tick++;
    }
    if (currentMove == null || tick > currentMove.path.length) {
        currentMove = simulator.step();
        tick = tickRate;
        board = nextBoard;
        nextBoard = simulator.board_data();
        tick = 0;
    }
    draw();
    window.requestAnimationFrame(gameLoop);
}

window.requestAnimationFrame(gameLoop);
