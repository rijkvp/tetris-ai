<script lang="ts">
    import WeightsControl from "$lib/WeightsControl.svelte";
    import { onMount } from "svelte";
    import init, {
        Simulator,
        Move,
        MoveResult,
        Weights,
        get_piece_rotation,
    } from "tetris-ai";

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

    let tickRate = 10;
    let tick = 0;

    let canvas: HTMLCanvasElement;
    let context: CanvasRenderingContext2D;

    let simulator: Simulator = new Simulator();

    let currentMove: MoveResult | undefined;
    let board: Uint8Array | null = null;
    let nextBoard: Uint8Array | null = null;

    function drawBoard(board: Uint8Array) {
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

    function drawOutline(pieceIdx: number, move: Move) {
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
                context.fillStyle = "#aaa";
                const px = (col + c) * CELL_SIZE;
                const py = (row + r) * CELL_SIZE;
                context.fillRect(px, py, CELL_SIZE, CELL_SIZE);
            }
        }
    }

    function drawPiece(pieceIdx: number, move: Move) {
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

        // draw move
        if (currentMove != null) {
            // currentMove.path.forEach((move) => {
            //     drawOutline(currentMove.piece_idx, move);
            // });
            const moveProgress = tick;
            drawPiece(
                currentMove.piece_idx,
                currentMove.path[
                    Math.min(moveProgress, currentMove.path.length - 1)
                ],
            );
        }
        // draw the board
        if (board !== null) drawBoard(board);
    }

    let lastFrameTime = 0;
    let tickAccumulator = 0;

    function gameLoop(currentTime: number) {
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

    function onWeightsChange(weights: Weights) {
        console.log(`updating weights`);
        simulator.update_weights(weights);
    }

    onMount(() => {
        context = canvas.getContext("2d")!;
        window.requestAnimationFrame(gameLoop);
    });
</script>

<h1>Tetris AI</h1>
<canvas bind:this={canvas} id="game" width="320" height="640"></canvas>

<div>
    <h2>Controls</h2>
    <input
        id="speed-input"
        type="range"
        bind:value={tickRate}
        min="1"
        max="1000"
    />
    <label for="speed-input">{tickRate} ticks per second</label>
    <WeightsControl {onWeightsChange} />
</div>

<style>
    canvas {
        border: 2px solid #444;
        background-color: #ddd;
    }
</style>
