
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @returns {number}
*/
export function width() {
    var ret = wasm.width();
    return ret >>> 0;
}

/**
* @returns {number}
*/
export function height() {
    var ret = wasm.height();
    return ret >>> 0;
}

/**
* @returns {number}
*/
export function render_stranger() {
    var ret = wasm.render_stranger();
    return ret;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8);
    getFloat64Memory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function getArrayF64FromWasm0(ptr, len) {
    return getFloat64Memory0().subarray(ptr / 8, ptr / 8 + len);
}
/**
* @param {Float64Array} pts
* @param {number | undefined} tension
* @param {number | undefined} num_of_segments
* @param {number | undefined} invert_x_with_width
* @param {number | undefined} invert_y_with_height
* @returns {Float64Array}
*/
export function getCurvePoints(pts, tension, num_of_segments, invert_x_with_width, invert_y_with_height) {
    var ptr0 = passArrayF64ToWasm0(pts, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.getCurvePoints(8, ptr0, len0, !isLikeNone(tension), isLikeNone(tension) ? 0 : tension, !isLikeNone(num_of_segments), isLikeNone(num_of_segments) ? 0 : num_of_segments, !isLikeNone(invert_x_with_width), isLikeNone(invert_x_with_width) ? 0 : invert_x_with_width, !isLikeNone(invert_y_with_height), isLikeNone(invert_y_with_height) ? 0 : invert_y_with_height);
    var r0 = getInt32Memory0()[8 / 4 + 0];
    var r1 = getInt32Memory0()[8 / 4 + 1];
    var v1 = getArrayF64FromWasm0(r0, r1).slice();
    wasm.__wbindgen_free(r0, r1 * 8);
    return v1;
}

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}
/**
*/
export class Metadata {

    static __wrap(ptr) {
        const obj = Object.create(Metadata.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_metadata_free(ptr);
    }
    /**
    * @returns {number}
    */
    get height() {
        var ret = wasm.__wbg_get_metadata_height(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set height(arg0) {
        wasm.__wbg_set_metadata_height(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get length() {
        var ret = wasm.__wbg_get_metadata_length(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set length(arg0) {
        wasm.__wbg_set_metadata_length(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get weight() {
        var ret = wasm.__wbg_get_metadata_weight(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set weight(arg0) {
        wasm.__wbg_set_metadata_weight(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get size_variance() {
        var ret = wasm.__wbg_get_metadata_size_variance(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set size_variance(arg0) {
        wasm.__wbg_set_metadata_size_variance(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get iq() {
        var ret = wasm.__wbg_get_metadata_iq(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set iq(arg0) {
        wasm.__wbg_set_metadata_iq(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get core_temp() {
        var ret = wasm.__wbg_get_metadata_core_temp(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set core_temp(arg0) {
        wasm.__wbg_set_metadata_core_temp(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get stability() {
        var ret = wasm.__wbg_get_metadata_stability(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set stability(arg0) {
        wasm.__wbg_set_metadata_stability(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get prevalence() {
        var ret = wasm.__wbg_get_metadata_prevalence(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set prevalence(arg0) {
        wasm.__wbg_set_metadata_prevalence(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get constancy() {
        var ret = wasm.__wbg_get_metadata_constancy(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set constancy(arg0) {
        wasm.__wbg_set_metadata_constancy(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get longevity() {
        var ret = wasm.__wbg_get_metadata_longevity(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set longevity(arg0) {
        wasm.__wbg_set_metadata_longevity(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get no_appearing() {
        var ret = wasm.__wbg_get_metadata_no_appearing(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set no_appearing(arg0) {
        wasm.__wbg_set_metadata_no_appearing(this.ptr, arg0);
    }
    /**
    */
    constructor() {
        var ret = wasm.metadata_new();
        return Metadata.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    get class_t() {
        try {
            wasm.metadata_class_t(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    get disposition() {
        try {
            wasm.metadata_disposition(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    get vision() {
        try {
            wasm.metadata_vision(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    get language_family() {
        try {
            wasm.metadata_language_family(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_getRandomValues_f5e14ab7ac8e995d = function(arg0, arg1, arg2) {
        getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));
    };
    imports.wbg.__wbg_randomFillSync_d5bd2d655fdf256a = function(arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    };
    imports.wbg.__wbg_self_1b7a39e3a92c949c = handleError(function() {
        var ret = self.self;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_require_604837428532a733 = function(arg0, arg1) {
        var ret = require(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_crypto_968f1772287e2df0 = function(arg0) {
        var ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_a3d34b4fee3c2869 = function(arg0) {
        var ret = getObject(arg0).getRandomValues;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

