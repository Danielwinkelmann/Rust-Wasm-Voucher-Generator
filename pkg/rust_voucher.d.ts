/* tslint:disable */
/* eslint-disable */
/**
* @param {string} pattern
* @param {number} count
* @param {boolean} char_set_number_enabled
* @param {boolean} char_set_symbol_enabled
* @param {boolean} char_set_uppercase_enabled
* @param {boolean} char_set_lowercase_enabled
* @returns {Array<any>}
*/
export function generate(pattern: string, count: number, char_set_number_enabled: boolean, char_set_symbol_enabled: boolean, char_set_uppercase_enabled: boolean, char_set_lowercase_enabled: boolean): Array<any>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly generate: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
