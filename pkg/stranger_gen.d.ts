/* tslint:disable */
/* eslint-disable */
/**
* @returns {number} 
*/
export function width(): number;
/**
* @returns {number} 
*/
export function height(): number;
/**
* @returns {number} 
*/
export function render_stranger(): number;
/**
* @param {Float64Array} pts 
* @param {number | undefined} tension 
* @param {number | undefined} num_of_segments 
* @param {number | undefined} invert_x_with_width 
* @param {number | undefined} invert_y_with_height 
* @returns {Float64Array} 
*/
export function getCurvePoints(pts: Float64Array, tension?: number, num_of_segments?: number, invert_x_with_width?: number, invert_y_with_height?: number): Float64Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly width: () => number;
  readonly height: () => number;
  readonly render_stranger: () => number;
  readonly getCurvePoints: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
        