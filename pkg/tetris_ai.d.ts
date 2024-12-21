/* tslint:disable */
/* eslint-disable */
export function get_piece_rotation(piece_idx: number, rotation: number): WasmPattern;
export function wasm_test(): any;
export class Move {
  private constructor();
  free(): void;
  rot: number;
  row: number;
  col: number;
}
export class MoveResult {
  private constructor();
  free(): void;
  piece_idx: number;
  path: (Move)[];
}
export class Pattern {
  private constructor();
  free(): void;
}
/**
 * A rotatable tetromino piece.
 */
export class Piece {
  private constructor();
  free(): void;
}
export class Simulator {
  free(): void;
  constructor();
  step(): MoveResult | undefined;
  board_data(): Uint8Array;
  generate_moves(piece_idx: number): (Move)[];
  steps: bigint;
}
export class WasmPattern {
  private constructor();
  free(): void;
  data: Uint8Array;
  size: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_move_free: (a: number, b: number) => void;
  readonly __wbg_get_move_rot: (a: number) => number;
  readonly __wbg_set_move_rot: (a: number, b: number) => void;
  readonly __wbg_get_move_row: (a: number) => number;
  readonly __wbg_set_move_row: (a: number, b: number) => void;
  readonly __wbg_get_move_col: (a: number) => number;
  readonly __wbg_set_move_col: (a: number, b: number) => void;
  readonly __wbg_piece_free: (a: number, b: number) => void;
  readonly __wbg_wasmpattern_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmpattern_data: (a: number) => [number, number];
  readonly __wbg_set_wasmpattern_data: (a: number, b: number, c: number) => void;
  readonly get_piece_rotation: (a: number, b: number) => number;
  readonly __wbg_pattern_free: (a: number, b: number) => void;
  readonly __wbg_simulator_free: (a: number, b: number) => void;
  readonly __wbg_get_simulator_steps: (a: number) => bigint;
  readonly __wbg_set_simulator_steps: (a: number, b: bigint) => void;
  readonly __wbg_moveresult_free: (a: number, b: number) => void;
  readonly __wbg_get_moveresult_piece_idx: (a: number) => number;
  readonly __wbg_set_moveresult_piece_idx: (a: number, b: number) => void;
  readonly __wbg_get_moveresult_path: (a: number) => [number, number];
  readonly __wbg_set_moveresult_path: (a: number, b: number, c: number) => void;
  readonly simulator_new: () => number;
  readonly simulator_step: (a: number) => number;
  readonly simulator_board_data: (a: number) => [number, number];
  readonly simulator_generate_moves: (a: number, b: number) => [number, number];
  readonly wasm_test: () => [number, number, number];
  readonly __wbg_get_wasmpattern_size: (a: number) => number;
  readonly __wbg_set_wasmpattern_size: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
