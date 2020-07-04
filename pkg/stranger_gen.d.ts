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
/**
*/
export class Metadata {
  free(): void;
/**
*/
  constructor();
/**
* @returns {string} 
*/
  readonly class_t: string;
/**
* @returns {number} 
*/
  constancy: number;
/**
* @returns {number} 
*/
  core_temp: number;
/**
* @returns {string} 
*/
  readonly disposition: string;
/**
* @returns {number} 
*/
  height: number;
/**
* @returns {number} 
*/
  iq: number;
/**
* @returns {string} 
*/
  readonly language_family: string;
/**
* @returns {number} 
*/
  length: number;
/**
* @returns {number} 
*/
  longevity: number;
/**
* @returns {number} 
*/
  no_appearing: number;
/**
* @returns {number} 
*/
  prevalence: number;
/**
* @returns {number} 
*/
  size_variance: number;
/**
* @returns {number} 
*/
  stability: number;
/**
* @returns {string} 
*/
  readonly vision: string;
/**
* @returns {number} 
*/
  weight: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_metadata_free: (a: number) => void;
  readonly __wbg_get_metadata_height: (a: number) => number;
  readonly __wbg_set_metadata_height: (a: number, b: number) => void;
  readonly __wbg_get_metadata_length: (a: number) => number;
  readonly __wbg_set_metadata_length: (a: number, b: number) => void;
  readonly __wbg_get_metadata_weight: (a: number) => number;
  readonly __wbg_set_metadata_weight: (a: number, b: number) => void;
  readonly __wbg_get_metadata_size_variance: (a: number) => number;
  readonly __wbg_set_metadata_size_variance: (a: number, b: number) => void;
  readonly __wbg_get_metadata_iq: (a: number) => number;
  readonly __wbg_set_metadata_iq: (a: number, b: number) => void;
  readonly __wbg_get_metadata_core_temp: (a: number) => number;
  readonly __wbg_set_metadata_core_temp: (a: number, b: number) => void;
  readonly __wbg_get_metadata_stability: (a: number) => number;
  readonly __wbg_set_metadata_stability: (a: number, b: number) => void;
  readonly __wbg_get_metadata_prevalence: (a: number) => number;
  readonly __wbg_set_metadata_prevalence: (a: number, b: number) => void;
  readonly __wbg_get_metadata_constancy: (a: number) => number;
  readonly __wbg_set_metadata_constancy: (a: number, b: number) => void;
  readonly __wbg_get_metadata_longevity: (a: number) => number;
  readonly __wbg_set_metadata_longevity: (a: number, b: number) => void;
  readonly __wbg_get_metadata_no_appearing: (a: number) => number;
  readonly __wbg_set_metadata_no_appearing: (a: number, b: number) => void;
  readonly width: () => number;
  readonly height: () => number;
  readonly render_stranger: () => number;
  readonly metadata_new: () => number;
  readonly metadata_class_t: (a: number, b: number) => void;
  readonly metadata_disposition: (a: number, b: number) => void;
  readonly metadata_vision: (a: number, b: number) => void;
  readonly metadata_language_family: (a: number, b: number) => void;
  readonly getCurvePoints: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
        