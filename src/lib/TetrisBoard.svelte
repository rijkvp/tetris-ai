<script lang="ts">
    import { onMount } from "svelte";
    import { Move } from "tetris-ai";
    import type { TetrisState } from "$lib/types.ts";
    import { clearBoard, displayBoard, displayCell } from "$lib/display";

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D;

    function displayMove(move: Move) {
        const pattern = move.get_pattern();

        for (let r = 0; r < pattern.size; r++) {
            for (let c = 0; c < pattern.size; c++) {
                const idx = r * pattern.size + c;
                if (!pattern.data[idx]) {
                    continue;
                }
                displayCell(
                    ctx,
                    move.pos.col + c,
                    move.pos.row + r,
                    move.piece.get_index(),
                );
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

    let {
        state = $bindable(),
        currentMove = $bindable(),
    }: { state: TetrisState; currentMove: Move | null } = $props();

    onMount(() => {
        ctx = canvas.getContext("2d")!;
    });

    export const clear = () => {
        clearBoard(ctx, canvas);
    };

    export const display = () => {
        clearBoard(ctx, canvas);
        if (state == null) {
            return;
        }
        displayBoard(ctx, state.board);
        if (currentMove != null) {
            displayMove(currentMove);
        }
        if (state.gameOver) {
            displayGameOver();
        }
    };
</script>

<canvas bind:this={canvas} class="border" width="320" height="640"></canvas>

<style>
    canvas {
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
