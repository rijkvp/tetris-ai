import init, { Simulator } from "./pkg/tetris_ai.js";

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

const canvas = document.getElementById('game');
const context = canvas.getContext('2d');

let updateDelay = 0.2;
const speedSlider = document.getElementById('speed-slider');
const speedLabel = document.getElementById('speed-label');
const stepLabel = document.getElementById('step-label');
speedSlider.addEventListener('input', () => {
    const speed = speedSlider.value;
    speedLabel.innerText = `${speed} pieces/sec`;
    updateDelay = 1 / speed;
});

const simulator = new Simulator();

function drawBoard() {
    context.clearRect(0, 0, canvas.width, canvas.height);
    const board = simulator.board_data();
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

let lastTimeStamp = 0;
let timeBeforeUpdate = 0;

function gameLoop(timeStamp) {
    let deltaTime = (timeStamp - lastTimeStamp) / 1000;
    lastTimeStamp = timeStamp;

    if (timeBeforeUpdate <= 0) {
        simulator.step();
        drawBoard();
        stepLabel.innerText = `Placed pieces: ${simulator.steps}`;
        timeBeforeUpdate = updateDelay;
    }
    timeBeforeUpdate -= deltaTime;
    window.requestAnimationFrame(gameLoop);
}

window.requestAnimationFrame(gameLoop);
