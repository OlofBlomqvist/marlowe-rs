let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}
/**
* @param {string} input
* @returns {string}
*/
export function decode_marlowe_data_or_redeemer(input) {
    let deferred3_0;
    let deferred3_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.decode_marlowe_data_or_redeemer(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        var ptr2 = r0;
        var len2 = r1;
        if (r3) {
            ptr2 = 0; len2 = 0;
            throw takeObject(r2);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred3_0, deferred3_1, 1);
    }
}

/**
* @param {string} dsl
* @returns {string}
*/
export function decode_marlowe_dsl_from_json(dsl) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(dsl, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.decode_marlowe_dsl_from_json(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred2_0, deferred2_1, 1);
    }
}

/**
* @param {string} redeemer_cbor_hex
* @returns {string}
*/
export function decode_marlowe_input_cbor_hex(redeemer_cbor_hex) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(redeemer_cbor_hex, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.decode_marlowe_input_cbor_hex(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred2_0, deferred2_1, 1);
    }
}

/**
* @param {bigint} x
* @returns {bigint}
*/
export function u64_to_i64(x) {
    const ret = wasm.u64_to_i64(x);
    return ret;
}

/**
* @param {bigint} x
* @returns {string}
*/
export function u64_to_string(x) {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.u64_to_string(retptr, x);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
    }
}

/**
* @param {bigint} x
* @returns {string}
*/
export function i64_to_string(x) {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.i64_to_string(retptr, x);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
    }
}

/**
*/
export function wasm_main() {
    wasm.wasm_main();
}

/**
* @param {string} contract
* @returns {string}
*/
export function marlowe_to_json(contract) {
    let deferred3_0;
    let deferred3_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(contract, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.marlowe_to_json(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        var ptr2 = r0;
        var len2 = r1;
        if (r3) {
            ptr2 = 0; len2 = 0;
            throw takeObject(r2);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred3_0, deferred3_1, 1);
    }
}

/**
* params_str format by example:
* "variable_one_name=12345,variable_two_name=6789"
* @param {string} contract
* @param {string} params_str
* @returns {string}
*/
export function marlowe_to_json_with_variables(contract, params_str) {
    let deferred4_0;
    let deferred4_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(contract, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(params_str, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        wasm.marlowe_to_json_with_variables(retptr, ptr0, len0, ptr1, len1);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        var ptr3 = r0;
        var len3 = r1;
        if (r3) {
            ptr3 = 0; len3 = 0;
            throw takeObject(r2);
        }
        deferred4_0 = ptr3;
        deferred4_1 = len3;
        return getStringFromWasm0(ptr3, len3);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred4_0, deferred4_1, 1);
    }
}

/**
* params_str format by example:
* "variable_one_name=12345,variable_two_name=6789"
* @param {string} contract
* @param {string} params_str
* @returns {string}
*/
export function parse_marlowe_with_variables(contract, params_str) {
    let deferred4_0;
    let deferred4_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(contract, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(params_str, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        wasm.parse_marlowe_with_variables(retptr, ptr0, len0, ptr1, len1);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        var ptr3 = r0;
        var len3 = r1;
        if (r3) {
            ptr3 = 0; len3 = 0;
            throw takeObject(r2);
        }
        deferred4_0 = ptr3;
        deferred4_1 = len3;
        return getStringFromWasm0(ptr3, len3);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred4_0, deferred4_1, 1);
    }
}

/**
* @param {string} contract
* @returns {string}
*/
export function format_marlowe(contract) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(contract, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.format_marlowe(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred2_0, deferred2_1, 1);
    }
}

/**
* @param {string} cbor_hex
* @returns {string}
*/
export function decode_cborhex_marlowe_plutus_datum(cbor_hex) {
    let deferred3_0;
    let deferred3_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(cbor_hex, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.decode_cborhex_marlowe_plutus_datum(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        var ptr2 = r0;
        var len2 = r1;
        if (r3) {
            ptr2 = 0; len2 = 0;
            throw takeObject(r2);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred3_0, deferred3_1, 1);
    }
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}
/**
* @param {string} marlowe_dsl
* @returns {any[]}
*/
export function get_input_params_for_contract(marlowe_dsl) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(marlowe_dsl, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.get_input_params_for_contract(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        if (r3) {
            throw takeObject(r2);
        }
        var v2 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export_0(r0, r1 * 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @param {string} marlowe_dsl
* @returns {ParseError | undefined}
*/
export function get_marlowe_dsl_parser_errors(marlowe_dsl) {
    const ptr0 = passStringToWasm0(marlowe_dsl, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_marlowe_dsl_parser_errors(ptr0, len0);
    return ret === 0 ? undefined : ParseError.__wrap(ret);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachedBigInt64Memory0 = null;

function getBigInt64Memory0() {
    if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
        cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
    }
    return cachedBigInt64Memory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
* @param {string} list_of_inputs_json
*/
export function redeemer_json_to_cbor(list_of_inputs_json) {
    const ptr0 = passStringToWasm0(list_of_inputs_json, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
    const len0 = WASM_VECTOR_LEN;
    wasm.redeemer_json_to_cbor(ptr0, len0);
}

/**
*/
export const WasmPartyType = Object.freeze({ Role:0,"0":"Role",Address:1,"1":"Address", });
/**
*/
export const WasmPayeeType = Object.freeze({ AccountRole:0,"0":"AccountRole",AccountAddress:1,"1":"AccountAddress",PartyRole:2,"2":"PartyRole",PartyAddress:3,"3":"PartyAddress", });
/**
*/
export const WasmTransactionWarningType = Object.freeze({ Failed:0,"0":"Failed",TransactionNonPositiveDeposit:1,"1":"TransactionNonPositiveDeposit",TransactionNonPositivePay:2,"2":"TransactionNonPositivePay",TransactionPartialPay:3,"3":"TransactionPartialPay",TransactionShadowing:4,"4":"TransactionShadowing", });
/**
*/
export const WasmMachineStateEnum = Object.freeze({ WaitingForInput:0,"0":"WaitingForInput",ReadyForNextStep:1,"1":"ReadyForNextStep",ContractHasTimedOut:2,"2":"ContractHasTimedOut",Closed:3,"3":"Closed",Faulted:4,"4":"Faulted", });
/**
*/
export class ObservationWasmResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ObservationWasmResult.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_observationwasmresult_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    get value() {
        const ret = wasm.__wbg_get_observationwasmresult_value(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set value(arg0) {
        wasm.__wbg_set_observationwasmresult_value(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {WasmTransactionWarnings}
    */
    get warnings() {
        const ret = wasm.__wbg_get_observationwasmresult_warnings(this.__wbg_ptr);
        return WasmTransactionWarnings.__wrap(ret);
    }
    /**
    * @param {WasmTransactionWarnings} arg0
    */
    set warnings(arg0) {
        _assertClass(arg0, WasmTransactionWarnings);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_observationwasmresult_warnings(this.__wbg_ptr, ptr0);
    }
}
/**
*/
export class ParseError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ParseError.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_parseerror_free(ptr);
    }
    /**
    * @returns {number}
    */
    get start_line() {
        const ret = wasm.__wbg_get_parseerror_start_line(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set start_line(arg0) {
        wasm.__wbg_set_parseerror_start_line(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get end_line() {
        const ret = wasm.__wbg_get_parseerror_end_line(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set end_line(arg0) {
        wasm.__wbg_set_parseerror_end_line(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get start_col() {
        const ret = wasm.__wbg_get_parseerror_start_col(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set start_col(arg0) {
        wasm.__wbg_set_parseerror_start_col(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get end_col() {
        const ret = wasm.__wbg_get_parseerror_end_col(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set end_col(arg0) {
        wasm.__wbg_set_parseerror_end_col(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {string}
    */
    get error_message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set error_message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @param {number} start_line
    * @param {number} end_line
    * @param {number} start_col
    * @param {number} end_col
    * @param {string} error_message
    */
    constructor(start_line, end_line, start_col, end_col, error_message) {
        const ptr0 = passStringToWasm0(error_message, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.parseerror_new(start_line, end_line, start_col, end_col, ptr0, len0);
        return ParseError.__wrap(ret);
    }
}
/**
*/
export class StringVec {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StringVec.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_stringvec_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {string}
    */
    get(n) {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stringvec_get(retptr, this.__wbg_ptr, n);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
}
/**
*/
export class WASMMarloweStateMachine {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WASMMarloweStateMachine.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmarlowestatemachine_free(ptr);
    }
    /**
    * @param {string} mintime
    */
    set_mintime(mintime) {
        const ptr0 = passStringToWasm0(mintime, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.wasmmarlowestatemachine_set_mintime(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
    * @param {string} datum_json
    * @returns {WASMMarloweStateMachine}
    */
    static from_datum_json(datum_json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(datum_json, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmmarlowestatemachine_from_datum_json(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return WASMMarloweStateMachine.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
    * @param {WasmDatum} datum
    * @returns {WASMMarloweStateMachine}
    */
    static from_datum(datum) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(datum, WasmDatum);
            var ptr0 = datum.__destroy_into_raw();
            wasm.wasmmarlowestatemachine_from_datum(retptr, ptr0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return WASMMarloweStateMachine.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Takes an initialized (non-marlowe-extended) MarloweDSL contract as input.
    * @param {string} contract_dsl
    * @param {string} role_payout_validator_hash
    */
    constructor(contract_dsl, role_payout_validator_hash) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(contract_dsl, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(role_payout_validator_hash, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
            const len1 = WASM_VECTOR_LEN;
            wasm.wasmmarlowestatemachine_new(retptr, ptr0, len0, ptr1, len1);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return WASMMarloweStateMachine.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {WasmDatum}
    */
    as_datum() {
        const ret = wasm.wasmmarlowestatemachine_as_datum(this.__wbg_ptr);
        return WasmDatum.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    datum_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmmarlowestatemachine_datum_json(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {string}
    */
    datum_text() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmmarlowestatemachine_datum_text(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {string}
    */
    get contract() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmmarlowestatemachine_contract(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {string}
    */
    timeout_continuation() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmmarlowestatemachine_timeout_continuation(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {StringVec}
    */
    logs() {
        const ret = wasm.wasmmarlowestatemachine_logs(this.__wbg_ptr);
        return StringVec.__wrap(ret);
    }
    /**
    * @returns {WasmPayments}
    */
    get payments() {
        const ret = wasm.wasmmarlowestatemachine_payments(this.__wbg_ptr);
        return WasmPayments.__wrap(ret);
    }
    /**
    * @returns {WasmState}
    */
    get state() {
        const ret = wasm.wasmmarlowestatemachine_state(this.__wbg_ptr);
        return WasmState.__wrap(ret);
    }
    /**
    * @returns {WasmTransactionWarnings}
    */
    warnings() {
        const ret = wasm.wasmmarlowestatemachine_warnings(this.__wbg_ptr);
        return WasmTransactionWarnings.__wrap(ret);
    }
    /**
    * @param {string} bech32_addr
    * @param {string} token_name
    * @param {string} currency_symbol
    * @param {string} quantity
    */
    set_acc_of_addr(bech32_addr, token_name, currency_symbol, quantity) {
        const ptr0 = passStringToWasm0(bech32_addr, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(token_name, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(currency_symbol, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(quantity, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len3 = WASM_VECTOR_LEN;
        wasm.wasmmarlowestatemachine_set_acc_of_addr(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    }
    /**
    * @param {string} role
    * @param {string} token_name
    * @param {string} currency_symbol
    * @param {string} quantity
    */
    set_acc_of_role(role, token_name, currency_symbol, quantity) {
        const ptr0 = passStringToWasm0(role, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(token_name, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(currency_symbol, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(quantity, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len3 = WASM_VECTOR_LEN;
        wasm.wasmmarlowestatemachine_set_acc_of_role(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    }
    /**
    * @returns {string}
    */
    describe() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmmarlowestatemachine_describe(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {WasmMachineState}
    */
    machine_state() {
        const ret = wasm.wasmmarlowestatemachine_machine_state(this.__wbg_ptr);
        return WasmMachineState.__wrap(ret);
    }
    /**
    * @param {string} from_role
    * @param {string} to_role
    * @param {string} token_name
    * @param {string} currency_symbol
    * @param {string} quantity
    */
    apply_input_deposit_for_role(from_role, to_role, token_name, currency_symbol, quantity) {
        const ptr0 = passStringToWasm0(from_role, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(to_role, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(token_name, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(currency_symbol, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len3 = WASM_VECTOR_LEN;
        const ptr4 = passStringToWasm0(quantity, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len4 = WASM_VECTOR_LEN;
        wasm.wasmmarlowestatemachine_apply_input_deposit_for_role(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    }
    /**
    * @param {string} from_bech32_addr
    * @param {string} to_bech32_addr
    * @param {string} token_name
    * @param {string} currency_symbol
    * @param {string} quantity
    */
    apply_input_deposit_for_addr(from_bech32_addr, to_bech32_addr, token_name, currency_symbol, quantity) {
        const ptr0 = passStringToWasm0(from_bech32_addr, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(to_bech32_addr, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(token_name, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(currency_symbol, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len3 = WASM_VECTOR_LEN;
        const ptr4 = passStringToWasm0(quantity, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len4 = WASM_VECTOR_LEN;
        wasm.wasmmarlowestatemachine_apply_input_deposit_for_addr(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    }
    /**
    * @param {string} choice_name
    * @param {string} role_name
    * @param {string} chosen_value
    */
    apply_input_choice_for_role(choice_name, role_name, chosen_value) {
        const ptr0 = passStringToWasm0(choice_name, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(role_name, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(chosen_value, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len2 = WASM_VECTOR_LEN;
        wasm.wasmmarlowestatemachine_apply_input_choice_for_role(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
    }
    /**
    * @param {string} choice_name
    * @param {string} bech32_addr
    * @param {string} chosen_value
    */
    apply_input_choice_for_addr(choice_name, bech32_addr, chosen_value) {
        const ptr0 = passStringToWasm0(choice_name, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(bech32_addr, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(chosen_value, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len2 = WASM_VECTOR_LEN;
        wasm.wasmmarlowestatemachine_apply_input_choice_for_addr(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
    }
    /**
    * @returns {string}
    */
    machine_state_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmmarlowestatemachine_machine_state_json(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} obs_json
    * @returns {ObservationWasmResult}
    */
    test_observation(obs_json) {
        const ptr0 = passStringToWasm0(obs_json, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmmarlowestatemachine_test_observation(this.__wbg_ptr, ptr0, len0);
        return ObservationWasmResult.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    process() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmmarlowestatemachine_process(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred2_0, deferred2_1, 1);
        }
    }
}
/**
*/
export class WasmAccount {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmAccount.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmaccount_free(ptr);
    }
    /**
    * @returns {WasmParty}
    */
    get party() {
        const ret = wasm.__wbg_get_wasmaccount_party(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set party(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmaccount_party(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmToken}
    */
    get token() {
        const ret = wasm.__wbg_get_wasmaccount_token(this.__wbg_ptr);
        return WasmToken.__wrap(ret);
    }
    /**
    * @param {WasmToken} arg0
    */
    set token(arg0) {
        _assertClass(arg0, WasmToken);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmaccount_token(this.__wbg_ptr, ptr0);
    }
    /**
    * BIG INTEGER (i128)
    * @returns {string}
    */
    get amount_u128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmaccount_amount_u128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * BIG INTEGER (i128)
    * @param {string} arg0
    */
    set amount_u128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmaccount_amount_u128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmAccounts {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmAccounts.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmaccounts_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {WasmAccount}
    */
    get(n) {
        const ret = wasm.wasmaccounts_get(this.__wbg_ptr, n);
        return WasmAccount.__wrap(ret);
    }
}
/**
*/
export class WasmBoundValue {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmBoundValue.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmboundvalue_free(ptr);
    }
    /**
    * @returns {string}
    */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * BIG INTEGER (i128)
    * @returns {string}
    */
    get value_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmboundvalue_value_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * BIG INTEGER (i128)
    * @param {string} arg0
    */
    set value_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmboundvalue_value_i128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmBoundValues {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmBoundValues.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmboundvalues_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {WasmBoundValue}
    */
    get(n) {
        const ret = wasm.wasmboundvalues_get(this.__wbg_ptr, n);
        return WasmBoundValue.__wrap(ret);
    }
}
/**
*/
export class WasmChoice {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmChoice.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmchoice_free(ptr);
    }
    /**
    * @returns {string}
    */
    get choice_name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set choice_name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {WasmParty}
    */
    get choice_owner() {
        const ret = wasm.__wbg_get_wasmchoice_choice_owner(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set choice_owner(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmchoice_choice_owner(this.__wbg_ptr, ptr0);
    }
    /**
    * BIG INTEGER (i128)
    * @returns {string}
    */
    get value_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmchoice_value_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * BIG INTEGER (i128)
    * @param {string} arg0
    */
    set value_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmchoice_value_i128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmChoices {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmChoices.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmchoices_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {WasmChoice}
    */
    get(n) {
        const ret = wasm.wasmchoices_get(this.__wbg_ptr, n);
        return WasmChoice.__wrap(ret);
    }
}
/**
*/
export class WasmDatum {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmDatum.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmdatum_free(ptr);
    }
    /**
    * @returns {WasmState}
    */
    get state() {
        const ret = wasm.__wbg_get_wasmdatum_state(this.__wbg_ptr);
        return WasmState.__wrap(ret);
    }
    /**
    * @param {WasmState} arg0
    */
    set state(arg0) {
        _assertClass(arg0, WasmState);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmdatum_state(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {string}
    */
    get payout_validator_hash() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmdatum_payout_validator_hash(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set payout_validator_hash(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmdatum_payout_validator_hash(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get contract_dsl() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmdatum_contract_dsl(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set contract_dsl(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmdatum_contract_dsl(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    to_cbor_hex() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmdatum_to_cbor_hex(retptr, ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
}
/**
*/
export class WasmInputChoice {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmInputChoice.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasminputchoice_free(ptr);
    }
    /**
    * @returns {string}
    */
    get choice_name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set choice_name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {WasmParty}
    */
    get who_is_allowed_to_make_the_choice() {
        const ret = wasm.__wbg_get_wasmchoice_choice_owner(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set who_is_allowed_to_make_the_choice(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmchoice_choice_owner(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {string}
    */
    get bounds() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmchoice_value_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set bounds(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmchoice_value_i128(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get continuation_dsl() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmaccount_amount_u128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set continuation_dsl(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmaccount_amount_u128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmInputChoices {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmInputChoices.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasminputchoices_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {WasmInputChoice}
    */
    get(n) {
        const ret = wasm.wasminputchoices_get(this.__wbg_ptr, n);
        return WasmInputChoice.__wrap(ret);
    }
}
/**
*/
export class WasmInputDeposit {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmInputDeposit.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasminputdeposit_free(ptr);
    }
    /**
    * @returns {WasmParty}
    */
    get who_is_expected_to_pay() {
        const ret = wasm.__wbg_get_wasminputdeposit_who_is_expected_to_pay(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set who_is_expected_to_pay(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasminputdeposit_who_is_expected_to_pay(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmToken}
    */
    get expected_asset_type() {
        const ret = wasm.__wbg_get_wasminputdeposit_expected_asset_type(this.__wbg_ptr);
        return WasmToken.__wrap(ret);
    }
    /**
    * @param {WasmToken} arg0
    */
    set expected_asset_type(arg0) {
        _assertClass(arg0, WasmToken);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasminputdeposit_expected_asset_type(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {bigint}
    */
    get expected_amount() {
        const ret = wasm.__wbg_get_wasminputdeposit_expected_amount(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {bigint} arg0
    */
    set expected_amount(arg0) {
        wasm.__wbg_set_wasminputdeposit_expected_amount(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {WasmPayee}
    */
    get expected_payee() {
        const ret = wasm.__wbg_get_wasminputdeposit_expected_payee(this.__wbg_ptr);
        return WasmPayee.__wrap(ret);
    }
    /**
    * @param {WasmPayee} arg0
    */
    set expected_payee(arg0) {
        _assertClass(arg0, WasmPayee);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasminputdeposit_expected_payee(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {string}
    */
    get continuation_dsl() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasminputdeposit_continuation_dsl(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set continuation_dsl(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasminputdeposit_continuation_dsl(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmInputDeposits {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmInputDeposits.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasminputdeposits_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {WasmInputDeposit}
    */
    get(n) {
        const ret = wasm.wasminputdeposits_get(this.__wbg_ptr, n);
        return WasmInputDeposit.__wrap(ret);
    }
}
/**
*/
export class WasmInputNotification {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmInputNotification.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasminputnotification_free(ptr);
    }
    /**
    * @returns {string}
    */
    get continuation() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set continuation(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get observation() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmboundvalue_value_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set observation(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmboundvalue_value_i128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmInputNotifications {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmInputNotifications.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasminputnotifications_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {WasmInputNotification}
    */
    get(n) {
        const ret = wasm.wasminputnotifications_get(this.__wbg_ptr, n);
        return WasmInputNotification.__wrap(ret);
    }
}
/**
*/
export class WasmMachineState {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmMachineState.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmachinestate_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    get waiting_for_notification() {
        const ret = wasm.__wbg_get_wasmmachinestate_waiting_for_notification(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set waiting_for_notification(arg0) {
        wasm.__wbg_set_wasmmachinestate_waiting_for_notification(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {WasmInputDeposits | undefined}
    */
    get expected_deposits() {
        const ret = wasm.__wbg_get_wasmmachinestate_expected_deposits(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmInputDeposits.__wrap(ret);
    }
    /**
    * @param {WasmInputDeposits | undefined} arg0
    */
    set expected_deposits(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, WasmInputDeposits);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_wasmmachinestate_expected_deposits(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmInputChoices | undefined}
    */
    get expected_choices() {
        const ret = wasm.__wbg_get_wasmmachinestate_expected_choices(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmInputChoices.__wrap(ret);
    }
    /**
    * @param {WasmInputChoices | undefined} arg0
    */
    set expected_choices(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, WasmInputChoices);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_wasmmachinestate_expected_choices(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmInputNotifications | undefined}
    */
    get expected_notifications() {
        const ret = wasm.__wbg_get_wasmmachinestate_expected_notifications(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmInputNotifications.__wrap(ret);
    }
    /**
    * @param {WasmInputNotifications | undefined} arg0
    */
    set expected_notifications(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, WasmInputNotifications);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_wasmmachinestate_expected_notifications(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {string | undefined}
    */
    get error() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmmachinestate_error(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export_0(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string | undefined} arg0
    */
    set error(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmmachinestate_error(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {bigint | undefined}
    */
    get next_timeout() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmmachinestate_next_timeout(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r2 = getBigInt64Memory0()[retptr / 8 + 1];
            return r0 === 0 ? undefined : r2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {bigint | undefined} arg0
    */
    set next_timeout(arg0) {
        wasm.__wbg_set_wasmmachinestate_next_timeout(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? BigInt(0) : arg0);
    }
    /**
    * @returns {number}
    */
    get typ() {
        const ret = wasm.__wbg_get_wasmmachinestate_typ(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set typ(arg0) {
        wasm.__wbg_set_wasmmachinestate_typ(this.__wbg_ptr, arg0);
    }
}
/**
*/
export class WasmParty {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmParty.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmparty_free(ptr);
    }
    /**
    * @returns {string}
    */
    value() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmparty_value(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {number}
    */
    typ() {
        const ret = wasm.wasmparty_typ(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {string} bech32_addr
    * @returns {WasmParty}
    */
    static new_addr(bech32_addr) {
        const ptr0 = passStringToWasm0(bech32_addr, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmparty_new_addr(ptr0, len0);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {string} role_token
    * @returns {WasmParty}
    */
    static new_role(role_token) {
        const ptr0 = passStringToWasm0(role_token, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmparty_new_role(ptr0, len0);
        return WasmParty.__wrap(ret);
    }
}
/**
*/
export class WasmPayee {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPayee.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpayee_free(ptr);
    }
    /**
    * @returns {number}
    */
    get typ() {
        const ret = wasm.__wbg_get_wasmpayee_typ(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set typ(arg0) {
        wasm.__wbg_set_wasmpayee_typ(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {string}
    */
    get val() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set val(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmPayment {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPayment.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpayment_free(ptr);
    }
    /**
    * @returns {WasmParty}
    */
    get from() {
        const ret = wasm.__wbg_get_wasmaccount_party(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set from(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmaccount_party(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmPayee}
    */
    get to() {
        const ret = wasm.__wbg_get_wasmpayment_to(this.__wbg_ptr);
        return WasmPayee.__wrap(ret);
    }
    /**
    * @param {WasmPayee} arg0
    */
    set to(arg0) {
        _assertClass(arg0, WasmPayee);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmpayment_to(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmToken}
    */
    get token() {
        const ret = wasm.__wbg_get_wasmpayment_token(this.__wbg_ptr);
        return WasmToken.__wrap(ret);
    }
    /**
    * @param {WasmToken} arg0
    */
    set token(arg0) {
        _assertClass(arg0, WasmToken);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmpayment_token(this.__wbg_ptr, ptr0);
    }
    /**
    * BIG INTEGER (i128)
    * @returns {string}
    */
    get amount_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmpayment_amount_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * BIG INTEGER (i128)
    * @param {string} arg0
    */
    set amount_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmpayment_amount_i128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmPayments {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPayments.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpayments_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {WasmPayment}
    */
    get(n) {
        const ret = wasm.wasmpayments_get(this.__wbg_ptr, n);
        return WasmPayment.__wrap(ret);
    }
}
/**
*/
export class WasmState {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmState.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmstate_free(ptr);
    }
    /**
    * @returns {WasmAccounts}
    */
    get accounts() {
        const ret = wasm.__wbg_get_wasmstate_accounts(this.__wbg_ptr);
        return WasmAccounts.__wrap(ret);
    }
    /**
    * @param {WasmAccounts} arg0
    */
    set accounts(arg0) {
        _assertClass(arg0, WasmAccounts);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmstate_accounts(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmChoices}
    */
    get choices() {
        const ret = wasm.__wbg_get_wasmstate_choices(this.__wbg_ptr);
        return WasmChoices.__wrap(ret);
    }
    /**
    * @param {WasmChoices} arg0
    */
    set choices(arg0) {
        _assertClass(arg0, WasmChoices);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmstate_choices(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmBoundValues}
    */
    get bound_values() {
        const ret = wasm.__wbg_get_wasmstate_bound_values(this.__wbg_ptr);
        return WasmBoundValues.__wrap(ret);
    }
    /**
    * @param {WasmBoundValues} arg0
    */
    set bound_values(arg0) {
        _assertClass(arg0, WasmBoundValues);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmstate_bound_values(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {bigint | undefined}
    */
    get min_time() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmstate_min_time(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r2 = getBigInt64Memory0()[retptr / 8 + 1];
            return r0 === 0 ? undefined : BigInt.asUintN(64, r2);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {bigint | undefined} arg0
    */
    set min_time(arg0) {
        wasm.__wbg_set_wasmstate_min_time(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? BigInt(0) : arg0);
    }
}
/**
*/
export class WasmToken {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmToken.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtoken_free(ptr);
    }
    /**
    * @returns {string}
    */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get pol() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmboundvalue_value_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set pol(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmboundvalue_value_i128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmTransactionWarning {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmTransactionWarning.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtransactionwarning_free(ptr);
    }
    /**
    * @returns {number}
    */
    get typ() {
        const ret = wasm.__wbg_get_wasmtransactionwarning_typ(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set typ(arg0) {
        wasm.__wbg_set_wasmtransactionwarning_typ(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {any}
    */
    get value() {
        const ret = wasm.__wbg_get_wasmtransactionwarning_value(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set value(arg0) {
        wasm.__wbg_set_wasmtransactionwarning_value(this.__wbg_ptr, addHeapObject(arg0));
    }
}
/**
*/
export class WasmTransactionWarningFailed {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmTransactionWarningFailed.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtransactionwarningfailed_free(ptr);
    }
    /**
    * @returns {string}
    */
    get value() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set value(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmTransactionWarningTransactionNonPositiveDeposit {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmTransactionWarningTransactionNonPositiveDeposit.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtransactionwarningtransactionnonpositivedeposit_free(ptr);
    }
    /**
    * BigInt i64
    * @returns {string}
    */
    get asked_to_deposit_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * BigInt i64
    * @param {string} arg0
    */
    set asked_to_deposit_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {WasmParty}
    */
    get in_account() {
        const ret = wasm.__wbg_get_wasmchoice_choice_owner(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set in_account(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmchoice_choice_owner(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmToken}
    */
    get of_token() {
        const ret = wasm.__wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_of_token(this.__wbg_ptr);
        return WasmToken.__wrap(ret);
    }
    /**
    * @param {WasmToken} arg0
    */
    set of_token(arg0) {
        _assertClass(arg0, WasmToken);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_of_token(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmParty}
    */
    get party() {
        const ret = wasm.__wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_party(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set party(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_party(this.__wbg_ptr, ptr0);
    }
}
/**
*/
export class WasmTransactionWarningTransactionPartialPay {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmTransactionWarningTransactionPartialPay.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtransactionwarningtransactionpartialpay_free(ptr);
    }
    /**
    * @returns {WasmParty}
    */
    get account() {
        const ret = wasm.__wbg_get_wasmaccount_party(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set account(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmaccount_party(this.__wbg_ptr, ptr0);
    }
    /**
    * BigInt i128
    * @returns {string}
    */
    get asked_to_pay_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmtransactionwarningtransactionpartialpay_asked_to_pay_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * BigInt i128
    * @param {string} arg0
    */
    set asked_to_pay_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmtransactionwarningtransactionpartialpay_asked_to_pay_i128(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {WasmToken}
    */
    get of_token() {
        const ret = wasm.__wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_of_token(this.__wbg_ptr);
        return WasmToken.__wrap(ret);
    }
    /**
    * @param {WasmToken} arg0
    */
    set of_token(arg0) {
        _assertClass(arg0, WasmToken);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_of_token(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmPayee}
    */
    get to_payee() {
        const ret = wasm.__wbg_get_wasmtransactionwarningtransactionpartialpay_to_payee(this.__wbg_ptr);
        return WasmPayee.__wrap(ret);
    }
    /**
    * @param {WasmPayee} arg0
    */
    set to_payee(arg0) {
        _assertClass(arg0, WasmPayee);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_party(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {string}
    */
    get but_only_paid_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmtransactionwarningtransactionpartialpay_but_only_paid_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set but_only_paid_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmtransactionwarningtransactionpartialpay_but_only_paid_i128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmTransactionWarningTransactionShadowing {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmTransactionWarningTransactionShadowing.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtransactionwarningtransactionshadowing_free(ptr);
    }
    /**
    * @returns {string}
    */
    get value_id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_parseerror_error_message(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set value_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_parseerror_error_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get had_value_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmboundvalue_value_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set had_value_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmboundvalue_value_i128(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get is_now_assigned_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmtransactionwarningtransactionshadowing_is_now_assigned_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set is_now_assigned_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmtransactionwarningtransactionshadowing_is_now_assigned_i128(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
export class WasmTransactionWarningTransactionTransactionNonPositivePay {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmTransactionWarningTransactionTransactionNonPositivePay.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtransactionwarningtransactiontransactionnonpositivepay_free(ptr);
    }
    /**
    * @returns {WasmParty}
    */
    get account() {
        const ret = wasm.__wbg_get_wasmaccount_party(this.__wbg_ptr);
        return WasmParty.__wrap(ret);
    }
    /**
    * @param {WasmParty} arg0
    */
    set account(arg0) {
        _assertClass(arg0, WasmParty);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmaccount_party(this.__wbg_ptr, ptr0);
    }
    /**
    * BigInt i64
    * @returns {string}
    */
    get asked_to_pay_i128() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wasmtransactionwarningtransactionpartialpay_asked_to_pay_i128(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * BigInt i64
    * @param {string} arg0
    */
    set asked_to_pay_i128(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmtransactionwarningtransactionpartialpay_asked_to_pay_i128(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {WasmToken}
    */
    get of_token() {
        const ret = wasm.__wbg_get_wasmtransactionwarningtransactionnonpositivedeposit_of_token(this.__wbg_ptr);
        return WasmToken.__wrap(ret);
    }
    /**
    * @param {WasmToken} arg0
    */
    set of_token(arg0) {
        _assertClass(arg0, WasmToken);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_of_token(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {WasmPayee}
    */
    get to_payee() {
        const ret = wasm.__wbg_get_wasmtransactionwarningtransactionpartialpay_to_payee(this.__wbg_ptr);
        return WasmPayee.__wrap(ret);
    }
    /**
    * @param {WasmPayee} arg0
    */
    set to_payee(arg0) {
        _assertClass(arg0, WasmPayee);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmtransactionwarningtransactionnonpositivedeposit_party(this.__wbg_ptr, ptr0);
    }
}
/**
*/
export class WasmTransactionWarnings {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmTransactionWarnings.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtransactionwarnings_free(ptr);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.stringvec_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} n
    * @returns {WasmTransactionWarning}
    */
    get(n) {
        const ret = wasm.wasmtransactionwarnings_get(this.__wbg_ptr, n);
        return WasmTransactionWarning.__wrap(ret);
    }
}

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbindgen_object_clone_ref(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

export function __wbg_parseerror_new(arg0) {
    const ret = ParseError.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbindgen_error_new(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbg_wasmtransactionwarningfailed_new(arg0) {
    const ret = WasmTransactionWarningFailed.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbg_wasmtransactionwarningtransactionshadowing_new(arg0) {
    const ret = WasmTransactionWarningTransactionShadowing.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbg_wasmtransactionwarningtransactionpartialpay_new(arg0) {
    const ret = WasmTransactionWarningTransactionPartialPay.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbg_wasmtransactionwarningtransactionnonpositivedeposit_new(arg0) {
    const ret = WasmTransactionWarningTransactionNonPositiveDeposit.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbg_wasmtransactionwarningtransactiontransactionnonpositivepay_new(arg0) {
    const ret = WasmTransactionWarningTransactionTransactionNonPositivePay.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbg_log_ed32fbdd3c7f149c(arg0) {
    console.log(getObject(arg0));
};

export function __wbg_new_abda76e883ba8a5f() {
    const ret = new Error();
    return addHeapObject(ret);
};

export function __wbg_stack_658279fe44541cf6(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

export function __wbg_error_f851667af71bcfc6(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_export_0(deferred0_0, deferred0_1, 1);
    }
};

export function __wbg_getTime_def258a47d72ac06(arg0) {
    const ret = getObject(arg0).getTime();
    return ret;
};

export function __wbg_new0_d8c0fd74ecfbd384() {
    const ret = new Date();
    return addHeapObject(ret);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

