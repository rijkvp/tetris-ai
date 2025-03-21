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

// export type Path = {
//     piece_idx: number;
//     path: Pos[][];
// };
//
// export type Move = {
//     piece_idx: number;
//     pos: Pos;
// }
//
// export type Pos = {
//     col: number;
//     row: number;
//     rot: number;
// };
