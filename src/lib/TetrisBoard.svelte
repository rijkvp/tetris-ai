<script lang="ts">
    import { onMount } from "svelte";
    import { get_piece_rotation } from "tetris-ai";
    import type { GameState, Move } from "$lib/types.ts";
    import StatsPanel from "$lib/StatsPanel.svelte";
    import { clearBoard, displayBoard, displayCell } from "$lib/display";
    import { prefersDarkMode } from "$lib/theme";

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D;

    function displayPiece(pieceIdx: number, move: Move) {
        const pattern = get_piece_rotation(pieceIdx, move.rot);

        for (let r = 0; r < pattern.size; r++) {
            for (let c = 0; c < pattern.size; c++) {
                const idx = r * pattern.size + c;
                if (!pattern.data[idx]) {
                    continue;
                }
                displayCell(ctx, move.col + c, move.row + r, pieceIdx);
            }
        }
    }

    function displayGameOver() {
        ctx.fillStyle = "rgba(0, 0, 0, 0.5)";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = "#fff";

        const fontFamily = window.getComputedStyle(document.body).fontFamily;
        ctx.font = `bold 2rem ${fontFamily}`;
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";
        ctx.fillText("GAME OVER", canvas.width / 2, canvas.height / 2);
    }

    let { statsPanel = $bindable() }: { statsPanel: StatsPanel } = $props();

    export const clear = () => {
        clearBoard(ctx, canvas);
    };

    // the state that is currently being displayed
    // TODO: Wrap inside a single object
    let gameState = $state(null) as GameState | null;
    let nextGameState = $state(null) as GameState | null;
    let animate = $state(false);
    let currTick = $state(0);
    let currTickProgress = $state(0);
    let currGameOver = $state(false);

    export const display = (state: GameState, gameOver: boolean) => {
        animate = false;
        gameState = state;
        nextGameState = null;
        currGameOver = gameOver;
    };

    export const displayTransition = (
        state: GameState,
        next: GameState,
        tick: number,
        tickProgress: number,
        gameOver: boolean,
    ) => {
        animate = true;
        gameState = state;
        nextGameState = next;
        currTick = tick;
        currTickProgress = tickProgress;
        currGameOver = gameOver;
    };

    onMount(() => {
        ctx = canvas.getContext("2d")!;
    });

    $effect(() => {
        clearBoard(ctx, canvas);
        if (gameState == null) {
            return;
        }
        if (!animate) {
            displayBoard(ctx, gameState.board);
            statsPanel.update(gameState.stats);
            if (currGameOver) {
                displayGameOver();
            }
        } else if (nextGameState != null) {
            displayBoard(ctx, gameState.board);

            const currentMove = nextGameState.move;
            if (currentMove != null) {
                const currentPath = currentMove.path[currTick];
                const currentPathIndex = Math.min(
                    Math.floor(currTickProgress * currentPath.length),
                    currentPath.length - 1,
                );
                displayPiece(
                    currentMove.piece_idx,
                    currentPath[currentPathIndex],
                );
            }

            if (currGameOver) {
                displayGameOver();
            }
            statsPanel.update(gameState.stats);
        }
    });
</script>

<canvas bind:this={canvas} width="320" height="640"></canvas>

<style>
    canvas {
        border: 2px solid #fff;
        border-radius: 0px;
        background-color: #bbb;
        box-shadow:
            0 4px 8px 0 rgba(0, 0, 0, 0.2),
            0 6px 20px 0 rgba(0, 0, 0, 0.19);
    }

    @media (prefers-color-scheme: dark) {
        canvas {
            background-color: #070707;
        }
    }
</style>
