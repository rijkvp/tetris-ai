<script lang="ts">
    import { onMount } from "svelte";
    import {
        BOARD_WIDTH,
        BOARD_HEIGHT,
        type BoardData,
        transitionArrow,
        highlightCell,
        crossOutCell,
        strikeThroughCell,
        highlightLine,
        highlightWell,
        clearBoard,
        displayBoard,
    } from "$lib/display";
    import * as boards from "$lib/example-boards";

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D;

    function displayColTrans(board: BoardData) {
        for (let c = 0; c < BOARD_WIDTH; c++) {
            for (let r = 0; r < BOARD_HEIGHT - 1; r++) {
                if (board[r][c] !== 0 && board[r + 1][c] === 0) {
                    transitionArrow(ctx, c, r, c, r + 1);
                } else if (board[r][c] === 0 && board[r + 1][c] !== 0) {
                    transitionArrow(ctx, c, r + 1, c, r);
                }
            }
        }
    }

    function displayRowTrans(board: BoardData) {
        for (let r = 0; r < BOARD_HEIGHT; r++) {
            for (let c = 0; c < BOARD_WIDTH - 1; c++) {
                if (board[r][c] !== 0 && board[r][c + 1] === 0) {
                    transitionArrow(ctx, c, r, c + 1, r);
                } else if (board[r][c] === 0 && board[r][c + 1] !== 0) {
                    transitionArrow(ctx, c + 1, r, c, r);
                }
            }
        }
    }

    function displayPits(board: BoardData, height: number[]) {
        for (let c = 0; c < BOARD_WIDTH; c++) {
            for (let r = BOARD_HEIGHT - height[c]; r < BOARD_HEIGHT; r++) {
                if (board[r][c] === 0) {
                    crossOutCell(ctx, c, r);
                }
            }
        }
    }

    function displayWells(height: number[]) {
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
        for (let c = 0; c < BOARD_WIDTH; c++) {
            highlightWell(ctx, c, BOARD_HEIGHT - height[c] - wells[c], BOARD_HEIGHT - height[c]);
        }
    }

    function displayLandingHeight() {
        for (let r = 13; r < 17; r++) highlightCell(ctx, 6, r);
        highlightLine(ctx, 0, 17, BOARD_WIDTH, 17);
    }

    function displayErodedCells() {
        for (let c = 0; c < BOARD_WIDTH; c++) {
            strikeThroughCell(ctx, c, 13);
        }
        for (let c = 4; c < 8; c++) {
            highlightCell(ctx, c, 13);
        }
    }

    function getHeight(board: BoardData): number[] {
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

    function selectBoard(feature: string): number[][] {
        switch (feature) {
            case "col_trans":
            case "row_trans":
                return boards.TRANS_BOARD;
            case "pits":
                return boards.HOLES_BOARD;
            case "landing_height":
                return boards.LANDING_HEIGHT_BOARD;
            case "eroded_cells":
                return boards.ERODED_BOARD;
            case "cuml_wells":
                return boards.WELLS_BOARD;
            default:
                throw new Error(`Unkown feature: ${feature}`);
        }
    }

    function displayFeature(
        feature: string,
        board: BoardData,
        height: number[],
    ) {
        switch (feature) {
            case "col_trans":
                return displayColTrans(board);
            case "row_trans":
                return displayRowTrans(board);
            case "pits":
                return displayPits(board, height);
            case "landing_height":
                return displayLandingHeight();
            case "eroded_cells":
                return displayErodedCells();
            case "cuml_wells":
                return displayWells(height);
        }
    }

    onMount(() => {
        ctx = canvas.getContext("2d")!;
    });

    let { feature }: { feature: string } = $props();

    $effect(() => {
        const board = selectBoard(feature).map((row) => new Uint8Array(row));
        const height = getHeight(board);
        clearBoard(ctx, canvas);
        displayBoard(ctx, board);
        displayFeature(feature, board, height);
    });
</script>

<canvas bind:this={canvas} class="border" width="320" height="640"></canvas>
