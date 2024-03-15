/* tslint:disable */
/* eslint-disable */
/**
*/
export class Board {
  free(): void;
/**
* @param {boolean} inverse
*/
  constructor(inverse: boolean);
/**
* @returns {(Choice)[]}
*/
  get_choices(): (Choice)[];
/**
* @param {number} x
* @param {number} y
* @returns {FinalScore | undefined}
*/
  pick(x: number, y: number): FinalScore | undefined;
/**
*/
  enemy: bigint;
/**
*/
  player: bigint;
}
/**
*/
export class Choice {
  free(): void;
/**
* @returns {Array<any>}
*/
  get_taken(): Array<any>;
/**
* @returns {Position}
*/
  get_pos(): Position;
}
/**
*/
export class FinalScore {
  free(): void;
/**
*/
  enemy: number;
/**
*/
  player: number;
}
/**
*/
export class Position {
  free(): void;
/**
*/
  x: number;
/**
*/
  y: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_board_free: (a: number) => void;
  readonly __wbg_get_board_player: (a: number) => number;
  readonly __wbg_set_board_player: (a: number, b: number) => void;
  readonly __wbg_get_board_enemy: (a: number) => number;
  readonly __wbg_set_board_enemy: (a: number, b: number) => void;
  readonly __wbg_finalscore_free: (a: number) => void;
  readonly __wbg_get_finalscore_player: (a: number) => number;
  readonly __wbg_set_finalscore_player: (a: number, b: number) => void;
  readonly __wbg_get_finalscore_enemy: (a: number) => number;
  readonly __wbg_set_finalscore_enemy: (a: number, b: number) => void;
  readonly board_new: (a: number) => number;
  readonly board_get_choices: (a: number, b: number) => void;
  readonly board_pick: (a: number, b: number, c: number) => number;
  readonly __wbg_choice_free: (a: number) => void;
  readonly choice_get_taken: (a: number) => number;
  readonly choice_get_pos: (a: number) => number;
  readonly __wbg_set_position_x: (a: number, b: number) => void;
  readonly __wbg_set_position_y: (a: number, b: number) => void;
  readonly __wbg_get_position_x: (a: number) => number;
  readonly __wbg_get_position_y: (a: number) => number;
  readonly __wbg_position_free: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
