let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

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
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

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
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_3.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_3.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_50(arg0, arg1, arg2) {
    wasm.closure534_externref_shim(arg0, arg1, arg2);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_2.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addToExternrefTable0(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}
/**
 * Generate a new key pair from a provisional private key.
 * @param {string} provisional_private_key
 * @returns {Promise<Key>}
 */
export function generate_key_from_provisional(provisional_private_key) {
    const ptr0 = passStringToWasm0(provisional_private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.generate_key_from_provisional(ptr0, len0);
    return ret;
}

/**
 * Function to take a backup before calling the deposit function of the liquidity contract.
 * You can also get the pubkey_salt_hash from the return value.
 * @param {Config} config
 * @param {string} private_key
 * @param {string} amount
 * @param {number} token_index
 * @returns {Promise<string>}
 */
export function prepare_deposit(config, private_key, amount, token_index) {
    _assertClass(config, Config);
    var ptr0 = config.__destroy_into_raw();
    const ptr1 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(amount, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.prepare_deposit(ptr0, ptr1, len1, ptr2, len2, token_index);
    return ret;
}

/**
 * Function to send a tx request to the block builder. The return value contains information to take a backup.
 * @param {Config} config
 * @param {string} block_builder_url
 * @param {string} private_key
 * @param {(JsTransfer)[]} transfers
 * @returns {Promise<TxRequestResult>}
 */
export function send_tx_request(config, block_builder_url, private_key, transfers) {
    _assertClass(config, Config);
    var ptr0 = config.__destroy_into_raw();
    const ptr1 = passStringToWasm0(block_builder_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayJsValueToWasm0(transfers, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ret = wasm.send_tx_request(ptr0, ptr1, len1, ptr2, len2, ptr3, len3);
    return ret;
}

/**
 * Function to query the block proposal from the block builder.
 * The return value is the block proposal or null if the proposal is not found.
 * If got an invalid proposal, it will return an error.
 * @param {Config} config
 * @param {string} block_builder_url
 * @param {string} private_key
 * @param {JsTx} tx
 * @returns {Promise<any>}
 */
export function query_proposal(config, block_builder_url, private_key, tx) {
    _assertClass(config, Config);
    var ptr0 = config.__destroy_into_raw();
    const ptr1 = passStringToWasm0(block_builder_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    _assertClass(tx, JsTx);
    const ret = wasm.query_proposal(ptr0, ptr1, len1, ptr2, len2, tx.__wbg_ptr);
    return ret;
}

/**
 * In this function, query block proposal from the block builder,
 * and then send the signed tx tree root to the block builder.
 * A backup of the tx is also taken.
 * You need to call send_tx_request before calling this function.
 * The return value is the tx tree root.
 * @param {Config} config
 * @param {string} block_builder_url
 * @param {string} private_key
 * @param {any} tx_request_memo
 * @param {any} proposal
 * @returns {Promise<string>}
 */
export function finalize_tx(config, block_builder_url, private_key, tx_request_memo, proposal) {
    _assertClass(config, Config);
    var ptr0 = config.__destroy_into_raw();
    const ptr1 = passStringToWasm0(block_builder_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.finalize_tx(ptr0, ptr1, len1, ptr2, len2, tx_request_memo, proposal);
    return ret;
}

/**
 * Synchronize the user's balance proof. It may take a long time to generate ZKP.
 * @param {Config} config
 * @param {string} private_key
 * @returns {Promise<void>}
 */
export function sync(config, private_key) {
    _assertClass(config, Config);
    var ptr0 = config.__destroy_into_raw();
    const ptr1 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.sync(ptr0, ptr1, len1);
    return ret;
}

/**
 * Synchronize the user's withdrawal proof, and send request to the withdrawal aggregator.
 * It may take a long time to generate ZKP.
 * @param {Config} config
 * @param {string} private_key
 * @returns {Promise<void>}
 */
export function sync_withdrawals(config, private_key) {
    _assertClass(config, Config);
    var ptr0 = config.__destroy_into_raw();
    const ptr1 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.sync_withdrawals(ptr0, ptr1, len1);
    return ret;
}

/**
 * Get the user's data. It is recommended to sync before calling this function.
 * @param {Config} config
 * @param {string} private_key
 * @returns {Promise<JsUserData>}
 */
export function get_user_data(config, private_key) {
    _assertClass(config, Config);
    var ptr0 = config.__destroy_into_raw();
    const ptr1 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.get_user_data(ptr0, ptr1, len1);
    return ret;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
 * Decrypt the deposit data.
 * @param {string} private_key
 * @param {Uint8Array} data
 * @returns {Promise<JsDepositData>}
 */
export function decrypt_deposit_data(private_key, data) {
    const ptr0 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.decrypt_deposit_data(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * Decrypt the transfer data. This is also used to decrypt the withdrawal data.
 * @param {string} private_key
 * @param {Uint8Array} data
 * @returns {Promise<JsTransferData>}
 */
export function decrypt_transfer_data(private_key, data) {
    const ptr0 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.decrypt_transfer_data(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * Decrypt the tx data.
 * @param {string} private_key
 * @param {Uint8Array} data
 * @returns {Promise<JsTxData>}
 */
export function decrypt_tx_data(private_key, data) {
    const ptr0 = passStringToWasm0(private_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.decrypt_tx_data(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * Function to mimic the deposit call of the contract. For development purposes only.
 * @param {string} contract_server_url
 * @param {string} pubkey_salt_hash
 * @param {string} amount
 * @returns {Promise<void>}
 */
export function mimic_deposit(contract_server_url, pubkey_salt_hash, amount) {
    const ptr0 = passStringToWasm0(contract_server_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(pubkey_salt_hash, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(amount, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.mimic_deposit(ptr0, len0, ptr1, len1, ptr2, len2);
    return ret;
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}
function __wbg_adapter_260(arg0, arg1, arg2, arg3) {
    wasm.closure651_externref_shim(arg0, arg1, arg2, arg3);
}

const __wbindgen_enum_BinaryType = ["blob", "arraybuffer"];

const __wbindgen_enum_RequestCredentials = ["omit", "same-origin", "include"];

const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];

const ConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_config_free(ptr >>> 0, 1));

export class Config {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Config.prototype);
        obj.__wbg_ptr = ptr;
        ConfigFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ConfigFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_config_free(ptr, 0);
    }
    /**
     * URL of the store vault server
     * @returns {string}
     */
    get store_vault_server_url() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_config_store_vault_server_url(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * URL of the store vault server
     * @param {string} arg0
     */
    set store_vault_server_url(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_config_store_vault_server_url(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * URL of the block validity prover
     * @returns {string}
     */
    get block_validity_prover_url() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_config_block_validity_prover_url(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * URL of the block validity prover
     * @param {string} arg0
     */
    set block_validity_prover_url(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_config_block_validity_prover_url(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * URL of the balance prover
     * @returns {string}
     */
    get balance_prover_url() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_config_balance_prover_url(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * URL of the balance prover
     * @param {string} arg0
     */
    set balance_prover_url(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_config_balance_prover_url(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * URL of the withdrawal aggregator
     * @returns {string}
     */
    get withdrawal_aggregator_url() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_config_withdrawal_aggregator_url(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * URL of the withdrawal aggregator
     * @param {string} arg0
     */
    set withdrawal_aggregator_url(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_config_withdrawal_aggregator_url(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Time to reach the rollup contract after taking a backup of the deposit
     * If this time is exceeded, the deposit backup will be ignored
     * @returns {bigint}
     */
    get deposit_timeout() {
        const ret = wasm.__wbg_get_config_deposit_timeout(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Time to reach the rollup contract after taking a backup of the deposit
     * If this time is exceeded, the deposit backup will be ignored
     * @param {bigint} arg0
     */
    set deposit_timeout(arg0) {
        wasm.__wbg_set_config_deposit_timeout(this.__wbg_ptr, arg0);
    }
    /**
     * Time to reach the rollup contract after sending a tx request
     * If this time is exceeded, the tx request will be ignored
     * @returns {bigint}
     */
    get tx_timeout() {
        const ret = wasm.__wbg_get_config_tx_timeout(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Time to reach the rollup contract after sending a tx request
     * If this time is exceeded, the tx request will be ignored
     * @param {bigint} arg0
     */
    set tx_timeout(arg0) {
        wasm.__wbg_set_config_tx_timeout(this.__wbg_ptr, arg0);
    }
    /**
     * @param {string} store_vault_server_url
     * @param {string} block_validity_prover_url
     * @param {string} balance_prover_url
     * @param {string} withdrawal_aggregator_url
     * @param {bigint} deposit_timeout
     * @param {bigint} tx_timeout
     * @returns {Config}
     */
    static new(store_vault_server_url, block_validity_prover_url, balance_prover_url, withdrawal_aggregator_url, deposit_timeout, tx_timeout) {
        const ptr0 = passStringToWasm0(store_vault_server_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(block_validity_prover_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(balance_prover_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(withdrawal_aggregator_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.config_new(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, deposit_timeout, tx_timeout);
        return Config.__wrap(ret);
    }
}

const JsDepositDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsdepositdata_free(ptr >>> 0, 1));

export class JsDepositData {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsDepositData.prototype);
        obj.__wbg_ptr = ptr;
        JsDepositDataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsDepositDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsdepositdata_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get deposit_salt() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jsdepositdata_deposit_salt(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set deposit_salt(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsdepositdata_deposit_salt(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get pubkey_salt_hash() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jsdepositdata_pubkey_salt_hash(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set pubkey_salt_hash(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsdepositdata_pubkey_salt_hash(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get token_index() {
        const ret = wasm.__wbg_get_jsdepositdata_token_index(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set token_index(arg0) {
        wasm.__wbg_set_jsdepositdata_token_index(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get amount() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jsdepositdata_amount(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set amount(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsdepositdata_amount(this.__wbg_ptr, ptr0, len0);
    }
}

const JsGenericAddressFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsgenericaddress_free(ptr >>> 0, 1));

export class JsGenericAddress {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsGenericAddress.prototype);
        obj.__wbg_ptr = ptr;
        JsGenericAddressFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsGenericAddressFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsgenericaddress_free(ptr, 0);
    }
    /**
     * @returns {boolean}
     */
    get is_pubkey() {
        const ret = wasm.__wbg_get_jsgenericaddress_is_pubkey(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set is_pubkey(arg0) {
        wasm.__wbg_set_jsgenericaddress_is_pubkey(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get data() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jsgenericaddress_data(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set data(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsdepositdata_deposit_salt(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {boolean} is_pubkey
     * @param {string} data
     */
    constructor(is_pubkey, data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jsgenericaddress_new(is_pubkey, ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        JsGenericAddressFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const JsTransferFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jstransfer_free(ptr >>> 0, 1));

export class JsTransfer {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsTransfer.prototype);
        obj.__wbg_ptr = ptr;
        JsTransferFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof JsTransfer)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsTransferFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jstransfer_free(ptr, 0);
    }
    /**
     * @returns {JsGenericAddress}
     */
    get recipient() {
        const ret = wasm.__wbg_get_jstransfer_recipient(this.__wbg_ptr);
        return JsGenericAddress.__wrap(ret);
    }
    /**
     * @param {JsGenericAddress} arg0
     */
    set recipient(arg0) {
        _assertClass(arg0, JsGenericAddress);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_jstransfer_recipient(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {number}
     */
    get token_index() {
        const ret = wasm.__wbg_get_jstransfer_token_index(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set token_index(arg0) {
        wasm.__wbg_set_jstransfer_token_index(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get amount() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jstransfer_amount(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set amount(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jstransfer_amount(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get salt() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jstransfer_salt(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set salt(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jstransfer_salt(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {JsGenericAddress} recipient
     * @param {number} token_index
     * @param {string} amount
     * @param {string} salt
     */
    constructor(recipient, token_index, amount, salt) {
        _assertClass(recipient, JsGenericAddress);
        var ptr0 = recipient.__destroy_into_raw();
        const ptr1 = passStringToWasm0(amount, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(salt, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.jstransfer_new(ptr0, token_index, ptr1, len1, ptr2, len2);
        this.__wbg_ptr = ret >>> 0;
        JsTransferFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const JsTransferDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jstransferdata_free(ptr >>> 0, 1));

export class JsTransferData {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsTransferData.prototype);
        obj.__wbg_ptr = ptr;
        JsTransferDataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsTransferDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jstransferdata_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get sender() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jstransferdata_sender(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set sender(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsdepositdata_deposit_salt(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {JsTransfer}
     */
    get transfer() {
        const ret = wasm.__wbg_get_jstransferdata_transfer(this.__wbg_ptr);
        return JsTransfer.__wrap(ret);
    }
    /**
     * @param {JsTransfer} arg0
     */
    set transfer(arg0) {
        _assertClass(arg0, JsTransfer);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_jstransferdata_transfer(this.__wbg_ptr, ptr0);
    }
}

const JsTxFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jstx_free(ptr >>> 0, 1));

export class JsTx {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsTx.prototype);
        obj.__wbg_ptr = ptr;
        JsTxFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsTxFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jstx_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get transfer_tree_root() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jstx_transfer_tree_root(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set transfer_tree_root(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsdepositdata_deposit_salt(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get nonce() {
        const ret = wasm.__wbg_get_jstx_nonce(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set nonce(arg0) {
        wasm.__wbg_set_jstx_nonce(this.__wbg_ptr, arg0);
    }
}

const JsTxDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jstxdata_free(ptr >>> 0, 1));

export class JsTxData {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsTxData.prototype);
        obj.__wbg_ptr = ptr;
        JsTxDataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsTxDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jstxdata_free(ptr, 0);
    }
    /**
     * @returns {JsTx}
     */
    get tx() {
        const ret = wasm.__wbg_get_jstxdata_tx(this.__wbg_ptr);
        return JsTx.__wrap(ret);
    }
    /**
     * @param {JsTx} arg0
     */
    set tx(arg0) {
        _assertClass(arg0, JsTx);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_jstransfer_recipient(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {(JsTransfer)[]}
     */
    get transfers() {
        const ret = wasm.__wbg_get_jstxdata_transfers(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {(JsTransfer)[]} arg0
     */
    set transfers(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jstxdata_transfers(this.__wbg_ptr, ptr0, len0);
    }
}

const JsUserDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsuserdata_free(ptr >>> 0, 1));

export class JsUserData {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsUserData.prototype);
        obj.__wbg_ptr = ptr;
        JsUserDataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsUserDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsuserdata_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get pubkey() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jsuserdata_pubkey(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set pubkey(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsuserdata_pubkey(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get block_number() {
        const ret = wasm.__wbg_get_jsuserdata_block_number(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set block_number(arg0) {
        wasm.__wbg_set_jsuserdata_block_number(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {(TokenBalance)[]}
     */
    get balances() {
        const ret = wasm.__wbg_get_jsuserdata_balances(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {(TokenBalance)[]} arg0
     */
    set balances(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsuserdata_balances(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get private_commitment() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_jsuserdata_private_commitment(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set private_commitment(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsuserdata_private_commitment(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {bigint}
     */
    get deposit_lpt() {
        const ret = wasm.__wbg_get_jsuserdata_deposit_lpt(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @param {bigint} arg0
     */
    set deposit_lpt(arg0) {
        wasm.__wbg_set_jsuserdata_deposit_lpt(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {bigint}
     */
    get transfer_lpt() {
        const ret = wasm.__wbg_get_jsuserdata_transfer_lpt(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @param {bigint} arg0
     */
    set transfer_lpt(arg0) {
        wasm.__wbg_set_jsuserdata_transfer_lpt(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {bigint}
     */
    get tx_lpt() {
        const ret = wasm.__wbg_get_jsuserdata_tx_lpt(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @param {bigint} arg0
     */
    set tx_lpt(arg0) {
        wasm.__wbg_set_jsuserdata_tx_lpt(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {bigint}
     */
    get withdrawal_lpt() {
        const ret = wasm.__wbg_get_jsuserdata_withdrawal_lpt(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @param {bigint} arg0
     */
    set withdrawal_lpt(arg0) {
        wasm.__wbg_set_jsuserdata_withdrawal_lpt(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {(string)[]}
     */
    get processed_deposit_uuids() {
        const ret = wasm.__wbg_get_jsuserdata_processed_deposit_uuids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {(string)[]} arg0
     */
    set processed_deposit_uuids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsuserdata_processed_deposit_uuids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {(string)[]}
     */
    get processed_transfer_uuids() {
        const ret = wasm.__wbg_get_jsuserdata_processed_transfer_uuids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {(string)[]} arg0
     */
    set processed_transfer_uuids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsuserdata_processed_transfer_uuids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {(string)[]}
     */
    get processed_tx_uuids() {
        const ret = wasm.__wbg_get_jsuserdata_processed_tx_uuids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {(string)[]} arg0
     */
    set processed_tx_uuids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsuserdata_processed_tx_uuids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {(string)[]}
     */
    get processed_withdrawal_uuids() {
        const ret = wasm.__wbg_get_jsuserdata_processed_withdrawal_uuids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {(string)[]} arg0
     */
    set processed_withdrawal_uuids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsuserdata_processed_withdrawal_uuids(this.__wbg_ptr, ptr0, len0);
    }
}

const KeyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_key_free(ptr >>> 0, 1));

export class Key {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Key.prototype);
        obj.__wbg_ptr = ptr;
        KeyFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KeyFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_key_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get privkey() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_key_privkey(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set privkey(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_key_privkey(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get pubkey() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_key_pubkey(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set pubkey(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_key_pubkey(this.__wbg_ptr, ptr0, len0);
    }
}

const TokenBalanceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tokenbalance_free(ptr >>> 0, 1));

export class TokenBalance {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TokenBalance.prototype);
        obj.__wbg_ptr = ptr;
        TokenBalanceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof TokenBalance)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TokenBalanceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tokenbalance_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get token_index() {
        const ret = wasm.__wbg_get_jstx_nonce(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set token_index(arg0) {
        wasm.__wbg_set_jstx_nonce(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get amount() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_tokenbalance_amount(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set amount(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jsdepositdata_deposit_salt(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {boolean}
     */
    get is_insufficient() {
        const ret = wasm.__wbg_get_tokenbalance_is_insufficient(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set is_insufficient(arg0) {
        wasm.__wbg_set_tokenbalance_is_insufficient(this.__wbg_ptr, arg0);
    }
}

const TxRequestResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_txrequestresult_free(ptr >>> 0, 1));

export class TxRequestResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TxRequestResult.prototype);
        obj.__wbg_ptr = ptr;
        TxRequestResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TxRequestResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_txrequestresult_free(ptr, 0);
    }
    /**
     * @returns {JsTx}
     */
    get tx() {
        const ret = wasm.__wbg_get_txrequestresult_tx(this.__wbg_ptr);
        return JsTx.__wrap(ret);
    }
    /**
     * @param {JsTx} arg0
     */
    set tx(arg0) {
        _assertClass(arg0, JsTx);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_txrequestresult_tx(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {any}
     */
    get memo() {
        const ret = wasm.__wbg_get_txrequestresult_memo(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {any} arg0
     */
    set memo(arg0) {
        wasm.__wbg_set_txrequestresult_memo(this.__wbg_ptr, arg0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_txrequestresult_new = function(arg0) {
        const ret = TxRequestResult.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbg_jsuserdata_new = function(arg0) {
        const ret = JsUserData.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_jsdepositdata_new = function(arg0) {
        const ret = JsDepositData.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_jstransferdata_new = function(arg0) {
        const ret = JsTransferData.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_jstxdata_new = function(arg0) {
        const ret = JsTxData.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_key_new = function(arg0) {
        const ret = Key.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = arg0;
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = arg0;
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = arg0 in arg1;
        return ret;
    };
    imports.wbg.__wbindgen_is_bigint = function(arg0) {
        const ret = typeof(arg0) === 'bigint';
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return ret;
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        const ret = arg0 === arg1;
        return ret;
    };
    imports.wbg.__wbg_tokenbalance_new = function(arg0) {
        const ret = TokenBalance.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_jstransfer_new = function(arg0) {
        const ret = JsTransfer.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbindgen_as_number = function(arg0) {
        const ret = +arg0;
        return ret;
    };
    imports.wbg.__wbg_jstransfer_unwrap = function(arg0) {
        const ret = JsTransfer.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_tokenbalance_unwrap = function(arg0) {
        const ret = TokenBalance.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
        const ret = arg0 == arg1;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbg_String_b9412f8799faab3e = function(arg0, arg1) {
        const ret = String(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbg_getwithrefkey_edc2c8960f0f1191 = function(arg0, arg1) {
        const ret = arg0[arg1];
        return ret;
    };
    imports.wbg.__wbg_set_f975102236d3c502 = function(arg0, arg1, arg2) {
        arg0[arg1] = arg2;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = arg0.original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbg_fetch_43b95fc3417395cd = typeof fetch == 'function' ? fetch : notDefined('fetch');
    imports.wbg.__wbg_queueMicrotask_848aa4969108a57e = function(arg0) {
        const ret = arg0.queueMicrotask;
        return ret;
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(arg0) === 'function';
        return ret;
    };
    imports.wbg.__wbg_queueMicrotask_c5419c06eab41e73 = typeof queueMicrotask == 'function' ? queueMicrotask : notDefined('queueMicrotask');
    imports.wbg.__wbg_fetch_1fdc4448ed9eec00 = function(arg0, arg1) {
        const ret = arg0.fetch(arg1);
        return ret;
    };
    imports.wbg.__wbg_newwithstrandinit_4b92c89af0a8e383 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = new Request(getStringFromWasm0(arg0, arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setbody_aa8b691bec428bf4 = function(arg0, arg1) {
        arg0.body = arg1;
    };
    imports.wbg.__wbg_setcredentials_a4e661320cdb9738 = function(arg0, arg1) {
        arg0.credentials = __wbindgen_enum_RequestCredentials[arg1];
    };
    imports.wbg.__wbg_setheaders_f5205d36e423a544 = function(arg0, arg1) {
        arg0.headers = arg1;
    };
    imports.wbg.__wbg_setmethod_ce2da76000b02f6a = function(arg0, arg1, arg2) {
        arg0.method = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setmode_4919fd636102c586 = function(arg0, arg1) {
        arg0.mode = __wbindgen_enum_RequestMode[arg1];
    };
    imports.wbg.__wbg_new_a9ae04a5200606a5 = function() { return handleError(function () {
        const ret = new Headers();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_append_8b3e7f74a47ea7d5 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.append(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_instanceof_Response_3c0e210a57ff751d = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Response;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_url_58af972663531d16 = function(arg0, arg1) {
        const ret = arg1.url;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_status_5f4e900d22140a18 = function(arg0) {
        const ret = arg0.status;
        return ret;
    };
    imports.wbg.__wbg_headers_1b9bf90c73fae600 = function(arg0) {
        const ret = arg0.headers;
        return ret;
    };
    imports.wbg.__wbg_arrayBuffer_144729e09879650e = function() { return handleError(function (arg0) {
        const ret = arg0.arrayBuffer();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_crypto_1d1f22824a6a080c = function(arg0) {
        const ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbg_process_4a72847cc503995b = function(arg0) {
        const ret = arg0.process;
        return ret;
    };
    imports.wbg.__wbg_versions_f686565e586dd935 = function(arg0) {
        const ret = arg0.versions;
        return ret;
    };
    imports.wbg.__wbg_node_104a2ff8d6ea03a2 = function(arg0) {
        const ret = arg0.node;
        return ret;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(arg0) === 'string';
        return ret;
    };
    imports.wbg.__wbg_require_cca90b1a94a0255b = function() { return handleError(function () {
        const ret = module.require;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_msCrypto_eb05e62b530a1508 = function(arg0) {
        const ret = arg0.msCrypto;
        return ret;
    };
    imports.wbg.__wbg_randomFillSync_5c9c955aa56b6049 = function() { return handleError(function (arg0, arg1) {
        arg0.randomFillSync(arg1);
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_3aa56aa6edec874c = function() { return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
    }, arguments) };
    imports.wbg.__wbg_get_5419cf6b954aa11d = function(arg0, arg1) {
        const ret = arg0[arg1 >>> 0];
        return ret;
    };
    imports.wbg.__wbg_length_f217bbbf7e8e4df4 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_new_034f913e7636e987 = function() {
        const ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_next_13b477da1eaa3897 = function(arg0) {
        const ret = arg0.next;
        return ret;
    };
    imports.wbg.__wbg_next_b06e115d1b01e10b = function() { return handleError(function (arg0) {
        const ret = arg0.next();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_done_983b5ffcaec8c583 = function(arg0) {
        const ret = arg0.done;
        return ret;
    };
    imports.wbg.__wbg_value_2ab8a198c834c26a = function(arg0) {
        const ret = arg0.value;
        return ret;
    };
    imports.wbg.__wbg_iterator_695d699a44d6234c = function() {
        const ret = Symbol.iterator;
        return ret;
    };
    imports.wbg.__wbg_get_ef828680c64da212 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(arg0, arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_a9ef466721e824f2 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_e69b5f66fda8f13c = function() {
        const ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_self_bf91bf94d9e04084 = function() { return handleError(function () {
        const ret = self.self;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_window_52dd9f07d03fd5f8 = function() { return handleError(function () {
        const ret = window.window;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_globalThis_05c129bf37fcf1be = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_global_3eca19bb09e9c484 = function() { return handleError(function () {
        const ret = global.global;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_425e70f7c64ac962 = function(arg0, arg1, arg2) {
        arg0[arg1 >>> 0] = arg2;
    };
    imports.wbg.__wbg_isArray_6f3b47f09adb61b5 = function(arg0) {
        const ret = Array.isArray(arg0);
        return ret;
    };
    imports.wbg.__wbg_instanceof_ArrayBuffer_74945570b4a62ec7 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof ArrayBuffer;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_call_3bfa248576352471 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.call(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_isSafeInteger_b9dff570f01a9100 = function(arg0) {
        const ret = Number.isSafeInteger(arg0);
        return ret;
    };
    imports.wbg.__wbg_getTime_41225036a0393d63 = function(arg0) {
        const ret = arg0.getTime();
        return ret;
    };
    imports.wbg.__wbg_new0_218ada33b570be35 = function() {
        const ret = new Date();
        return ret;
    };
    imports.wbg.__wbg_new_1073970097e5a420 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_260(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return ret;
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_0aad7c1484731c99 = function(arg0) {
        const ret = Promise.resolve(arg0);
        return ret;
    };
    imports.wbg.__wbg_then_748f75edfb032440 = function(arg0, arg1) {
        const ret = arg0.then(arg1);
        return ret;
    };
    imports.wbg.__wbg_then_4866a7d9f55d8f3e = function(arg0, arg1, arg2) {
        const ret = arg0.then(arg1, arg2);
        return ret;
    };
    imports.wbg.__wbg_buffer_ccaed51a635d8a2d = function(arg0) {
        const ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_7e3eb787208af730 = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_new_fec2611eb9180f95 = function(arg0) {
        const ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_set_ec2fcf81bc573fd9 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_length_9254c4bd3b9f23c4 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Uint8Array_df0761410414ef36 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Uint8Array;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_76462a666eca145f = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_subarray_975a06f9dbd16995 = function(arg0, arg1, arg2) {
        const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_stringify_eead5648c09faaf8 = function() { return handleError(function (arg0) {
        const ret = JSON.stringify(arg0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_has_bd717f25f195f23d = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.has(arg0, arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
        const v = arg1;
        const ret = typeof(v) === 'bigint' ? v : undefined;
        getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper1879 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 535, __wbg_adapter_50);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('intmax2_wasm_lib_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
