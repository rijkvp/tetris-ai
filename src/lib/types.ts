export type GameState = {
    board: Uint8Array[];
    stats: Stats;
    game_over: boolean;
};

export type Stats = {
    steps: bigint;
    lines: bigint;
    score: bigint;
    level: bigint;
    tetrises: bigint;
};

