export type TetrisState = {
    board: Uint8Array[];
    stats: Stats;
    gameOver: boolean;
}

export type Stats = {
    steps: bigint;
    lines: bigint;
    score: bigint;
    level: bigint;
    tetrises: bigint;
};

