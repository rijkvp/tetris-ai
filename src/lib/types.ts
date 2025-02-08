export type GameState = {
    board: Uint8Array[];
    stats: Stats;
    move: MoveResult;
};

export type Stats = {
    steps: bigint;
    lines: bigint;
    score: bigint;
    level: bigint;
};

export type MoveResult = {
    piece_idx: number;
    path: Move[][];
};

export type Move = {
    col: number;
    row: number;
    rot: number;
};
