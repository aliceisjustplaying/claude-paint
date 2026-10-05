/* @ts-self-types="./stylua_lib.d.ts" */

/**
 * If blocks should be allowed to have leading and trailing newline gaps.
 * @enum {0 | 1}
 */
export const BlockNewlineGaps = Object.freeze({
    /**
     * Never allow leading or trailing newline gaps
     */
    Never: 0, "0": "Never",
    /**
     * Preserve both leading and trailing newline gaps if present in input
     */
    Preserve: 1, "1": "Preserve",
});

/**
 * When to use call parentheses
 * @enum {0 | 1 | 2 | 3 | 4}
 */
export const CallParenType = Object.freeze({
    /**
     * Use call parentheses all the time
     */
    Always: 0, "0": "Always",
    /**
     * Skip call parentheses when only a string argument is used.
     */
    NoSingleString: 1, "1": "NoSingleString",
    /**
     * Skip call parentheses when only a table argument is used.
     */
    NoSingleTable: 2, "2": "NoSingleTable",
    /**
     * Skip call parentheses when only a table or string argument is used.
     */
    None: 3, "3": "None",
    /**
     * Keep call parentheses based on its presence in input code.
     */
    Input: 4, "4": "Input",
});

/**
 * What mode to use if we want to collapse simple functions / guard statements
 * @enum {0 | 1 | 2 | 3}
 */
export const CollapseSimpleStatement = Object.freeze({
    /**
     * Never collapse
     */
    Never: 0, "0": "Never",
    /**
     * Collapse simple functions onto a single line
     */
    FunctionOnly: 1, "1": "FunctionOnly",
    /**
     * Collapse simple if guards onto a single line
     */
    ConditionalOnly: 2, "2": "ConditionalOnly",
    /**
     * Collapse all simple statements onto a single line
     */
    Always: 3, "3": "Always",
});

/**
 * The configuration to use when formatting.
 */
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
     * Creates a new Config with the default values
     * @returns {Config}
     */
    static new() {
        const ret = wasm.config_new();
        return Config.__wrap(ret);
    }
    /**
     * Whether we should allow blocks to preserve leading and trailing newline gaps.
     * if set to [`BlockNewlineGaps::Never`] then newline gaps are never allowed at the start or end of blocks.
     * if set to [`BlockNewlineGaps::Preserve`] then newline gaps are preserved at the start and end of blocks.
     * @returns {BlockNewlineGaps}
     */
    get block_newline_gaps() {
        const ret = wasm.__wbg_get_config_block_newline_gaps(this.__wbg_ptr);
        return ret;
    }
    /**
     * When to use call parentheses.
     * if call_parentheses is set to [`CallParenType::Always`] call parentheses is always applied.
     * if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
     * function is called with only one string argument.
     * if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
     * function is called with only one table argument.
     * if call_parentheses is set to [`CallParenType::None`] call parentheses is omitted when
     * function is called with only one table or string argument (same as no_call_parentheses).
     * @returns {CallParenType}
     */
    get call_parentheses() {
        const ret = wasm.__wbg_get_config_call_parentheses(this.__wbg_ptr);
        return ret;
    }
    /**
     * Whether we should collapse simple structures like functions or guard statements
     * if set to [`CollapseSimpleStatement::None`] structures are never collapsed.
     * if set to [`CollapseSimpleStatement::FunctionOnly`] then simple functions (i.e., functions with a single laststmt) can be collapsed
     * @returns {CollapseSimpleStatement}
     */
    get collapse_simple_statement() {
        const ret = wasm.__wbg_get_config_collapse_simple_statement(this.__wbg_ptr);
        return ret;
    }
    /**
     * The approximate line length to use when printing the code.
     * This is used as a guide to determine when to wrap lines, but note
     * that this is not a hard upper bound.
     * @returns {number}
     */
    get column_width() {
        const ret = wasm.__wbg_get_config_column_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * The type of indents to use.
     * @returns {IndentType}
     */
    get indent_type() {
        const ret = wasm.__wbg_get_config_indent_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * The width of a single indentation level.
     * If `indent_type` is set to [`IndentType::Spaces`], then this is the number of spaces to use.
     * If `indent_type` is set to [`IndentType::Tabs`], then this is used as a heuristic to guide when to wrap lines.
     * @returns {number}
     */
    get indent_width() {
        const ret = wasm.__wbg_get_config_indent_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * The type of line endings to use.
     * @returns {LineEndings}
     */
    get line_endings() {
        const ret = wasm.__wbg_get_config_line_endings(this.__wbg_ptr);
        return ret;
    }
    /**
     * Whether to omit parentheses around function calls which take a single string literal or table.
     * This is added for adoption reasons only, and is not recommended for new work.
     * @returns {boolean}
     */
    get no_call_parentheses() {
        const ret = wasm.__wbg_get_config_no_call_parentheses(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * The style of quotes to use in string literals.
     * @returns {QuoteStyle}
     */
    get quote_style() {
        const ret = wasm.__wbg_get_config_quote_style(this.__wbg_ptr);
        return ret;
    }
    /**
     * Configuration for the sort requires codemod
     * @returns {SortRequiresConfig}
     */
    get sort_requires() {
        const ret = wasm.__wbg_get_config_sort_requires(this.__wbg_ptr);
        return SortRequiresConfig.__wrap(ret);
    }
    /**
     * Whether we should include a space between the function name and arguments.
     * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Never`] a space is never used.
     * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Definitions`] a space is used only for definitions.
     * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Calls`] a space is used only for calls.
     * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Always`] a space is used for both definitions and calls.
     * @returns {SpaceAfterFunctionNames}
     */
    get space_after_function_names() {
        const ret = wasm.__wbg_get_config_space_after_function_names(this.__wbg_ptr);
        return ret;
    }
    /**
     * The type of Lua syntax to parse.
     * @returns {LuaVersion}
     */
    get syntax() {
        const ret = wasm.__wbg_get_config_syntax(this.__wbg_ptr);
        return ret;
    }
    /**
     * Whether we should allow blocks to preserve leading and trailing newline gaps.
     * if set to [`BlockNewlineGaps::Never`] then newline gaps are never allowed at the start or end of blocks.
     * if set to [`BlockNewlineGaps::Preserve`] then newline gaps are preserved at the start and end of blocks.
     * @param {BlockNewlineGaps} arg0
     */
    set block_newline_gaps(arg0) {
        wasm.__wbg_set_config_block_newline_gaps(this.__wbg_ptr, arg0);
    }
    /**
     * When to use call parentheses.
     * if call_parentheses is set to [`CallParenType::Always`] call parentheses is always applied.
     * if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
     * function is called with only one string argument.
     * if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
     * function is called with only one table argument.
     * if call_parentheses is set to [`CallParenType::None`] call parentheses is omitted when
     * function is called with only one table or string argument (same as no_call_parentheses).
     * @param {CallParenType} arg0
     */
    set call_parentheses(arg0) {
        wasm.__wbg_set_config_call_parentheses(this.__wbg_ptr, arg0);
    }
    /**
     * Whether we should collapse simple structures like functions or guard statements
     * if set to [`CollapseSimpleStatement::None`] structures are never collapsed.
     * if set to [`CollapseSimpleStatement::FunctionOnly`] then simple functions (i.e., functions with a single laststmt) can be collapsed
     * @param {CollapseSimpleStatement} arg0
     */
    set collapse_simple_statement(arg0) {
        wasm.__wbg_set_config_collapse_simple_statement(this.__wbg_ptr, arg0);
    }
    /**
     * The approximate line length to use when printing the code.
     * This is used as a guide to determine when to wrap lines, but note
     * that this is not a hard upper bound.
     * @param {number} arg0
     */
    set column_width(arg0) {
        wasm.__wbg_set_config_column_width(this.__wbg_ptr, arg0);
    }
    /**
     * The type of indents to use.
     * @param {IndentType} arg0
     */
    set indent_type(arg0) {
        wasm.__wbg_set_config_indent_type(this.__wbg_ptr, arg0);
    }
    /**
     * The width of a single indentation level.
     * If `indent_type` is set to [`IndentType::Spaces`], then this is the number of spaces to use.
     * If `indent_type` is set to [`IndentType::Tabs`], then this is used as a heuristic to guide when to wrap lines.
     * @param {number} arg0
     */
    set indent_width(arg0) {
        wasm.__wbg_set_config_indent_width(this.__wbg_ptr, arg0);
    }
    /**
     * The type of line endings to use.
     * @param {LineEndings} arg0
     */
    set line_endings(arg0) {
        wasm.__wbg_set_config_line_endings(this.__wbg_ptr, arg0);
    }
    /**
     * Whether to omit parentheses around function calls which take a single string literal or table.
     * This is added for adoption reasons only, and is not recommended for new work.
     * @param {boolean} arg0
     */
    set no_call_parentheses(arg0) {
        wasm.__wbg_set_config_no_call_parentheses(this.__wbg_ptr, arg0);
    }
    /**
     * The style of quotes to use in string literals.
     * @param {QuoteStyle} arg0
     */
    set quote_style(arg0) {
        wasm.__wbg_set_config_quote_style(this.__wbg_ptr, arg0);
    }
    /**
     * Configuration for the sort requires codemod
     * @param {SortRequiresConfig} arg0
     */
    set sort_requires(arg0) {
        _assertClass(arg0, SortRequiresConfig);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_config_sort_requires(this.__wbg_ptr, ptr0);
    }
    /**
     * Whether we should include a space between the function name and arguments.
     * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Never`] a space is never used.
     * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Definitions`] a space is used only for definitions.
     * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Calls`] a space is used only for calls.
     * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Always`] a space is used for both definitions and calls.
     * @param {SpaceAfterFunctionNames} arg0
     */
    set space_after_function_names(arg0) {
        wasm.__wbg_set_config_space_after_function_names(this.__wbg_ptr, arg0);
    }
    /**
     * The type of Lua syntax to parse.
     * @param {LuaVersion} arg0
     */
    set syntax(arg0) {
        wasm.__wbg_set_config_syntax(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Config.prototype[Symbol.dispose] = Config.prototype.free;

/**
 * The type of indents to use when indenting
 * @enum {0 | 1}
 */
export const IndentType = Object.freeze({
    /**
     * Indent using tabs (`\t`)
     */
    Tabs: 0, "0": "Tabs",
    /**
     * Indent using spaces (` `)
     */
    Spaces: 1, "1": "Spaces",
});

/**
 * The type of line endings to use at the end of a line
 * @enum {0 | 1}
 */
export const LineEndings = Object.freeze({
    /**
     * Unix Line Endings (LF) - `\n`
     */
    Unix: 0, "0": "Unix",
    /**
     * Windows Line Endings (CRLF) - `\r\n`
     */
    Windows: 1, "1": "Windows",
});

/**
 * The Lua syntax version to use
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7}
 */
export const LuaVersion = Object.freeze({
    /**
     * Parse all syntax versions at the same time. This allows most general usage.
     * For overlapping syntaxes (e.g., Lua5.2 label syntax and Luau type assertions), select a
     * specific syntax version
     */
    All: 0, "0": "All",
    /**
     * Parse Lua 5.1 code
     */
    Lua51: 1, "1": "Lua51",
    /**
     * Parse Lua 5.2 code
     */
    Lua52: 2, "2": "Lua52",
    /**
     * Parse Lua 5.3 code
     */
    Lua53: 3, "3": "Lua53",
    /**
     * Parse Lua 5.4 code
     */
    Lua54: 4, "4": "Lua54",
    /**
     * Parse Luau code
     */
    Luau: 5, "5": "Luau",
    /**
     * Parse LuaJIT code
     */
    LuaJIT: 6, "6": "LuaJIT",
    /**
     * Parse Cfx Lua code
     */
    CfxLua: 7, "7": "CfxLua",
});

/**
 * The type of verification to perform to validate that the output AST is still correct.
 * @enum {0 | 1}
 */
export const OutputVerification = Object.freeze({
    /**
     * Reparse the generated output to detect any changes to code correctness.
     */
    Full: 0, "0": "Full",
    /**
     * Perform no verification of the output.
     */
    None: 1, "1": "None",
});

/**
 * The style of quotes to use within string literals
 * @enum {0 | 1 | 2 | 3}
 */
export const QuoteStyle = Object.freeze({
    /**
     * Use double quotes where possible, but change to single quotes if it produces less escapes
     */
    AutoPreferDouble: 0, "0": "AutoPreferDouble",
    /**
     * Use single quotes where possible, but change to double quotes if it produces less escapes
     */
    AutoPreferSingle: 1, "1": "AutoPreferSingle",
    /**
     * Always use double quotes in all strings
     */
    ForceDouble: 2, "2": "ForceDouble",
    /**
     * Always use single quotes in all strings
     */
    ForceSingle: 3, "3": "ForceSingle",
});

/**
 * An optional formatting range.
 * If provided, only content within these boundaries (inclusive) will be formatted.
 * Both boundaries are optional, and are given as byte offsets from the beginning of the file.
 */
export class Range {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Range.prototype);
        obj.__wbg_ptr = ptr;
        RangeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RangeFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_range_free(ptr, 0);
    }
    /**
     * @returns {number | undefined}
     */
    get end() {
        const ret = wasm.__wbg_get_range_end(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    get start() {
        const ret = wasm.__wbg_get_range_start(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * Creates a new formatting range from the given start and end point.
     * All content within these boundaries (inclusive) will be formatted.
     * @param {number | null} [start]
     * @param {number | null} [end]
     * @returns {Range}
     */
    static from_values(start, end) {
        const ret = wasm.range_from_values(isLikeNone(start) ? 0x100000001 : (start) >>> 0, isLikeNone(end) ? 0x100000001 : (end) >>> 0);
        return Range.__wrap(ret);
    }
    /**
     * @param {number | null} [arg0]
     */
    set end(arg0) {
        wasm.__wbg_set_range_end(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @param {number | null} [arg0]
     */
    set start(arg0) {
        wasm.__wbg_set_range_start(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
}
if (Symbol.dispose) Range.prototype[Symbol.dispose] = Range.prototype.free;

/**
 * Configuration for the Sort Requires codemod
 */
export class SortRequiresConfig {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SortRequiresConfig.prototype);
        obj.__wbg_ptr = ptr;
        SortRequiresConfigFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SortRequiresConfigFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sortrequiresconfig_free(ptr, 0);
    }
    /**
     * Whether the sort requires codemod is enabled
     * @returns {boolean}
     */
    get enabled() {
        const ret = wasm.__wbg_get_sortrequiresconfig_enabled(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Whether the sort requires codemod is enabled
     * @param {boolean} arg0
     */
    set enabled(arg0) {
        wasm.__wbg_set_sortrequiresconfig_enabled(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {SortRequiresConfig}
     */
    static new() {
        const ret = wasm.sortrequiresconfig_new();
        return SortRequiresConfig.__wrap(ret);
    }
    /**
     * @param {boolean} enabled
     * @returns {SortRequiresConfig}
     */
    set_enabled(enabled) {
        const ret = wasm.sortrequiresconfig_set_enabled(this.__wbg_ptr, enabled);
        return SortRequiresConfig.__wrap(ret);
    }
}
if (Symbol.dispose) SortRequiresConfig.prototype[Symbol.dispose] = SortRequiresConfig.prototype.free;

/**
 * When to use spaces after function names
 * @enum {0 | 1 | 2 | 3}
 */
export const SpaceAfterFunctionNames = Object.freeze({
    /**
     * Never use spaces after function names.
     */
    Never: 0, "0": "Never",
    /**
     * Use spaces after function names only for function definitions.
     */
    Definitions: 1, "1": "Definitions",
    /**
     * Use spaces after function names only for function calls.
     */
    Calls: 2, "2": "Calls",
    /**
     * Use spaces after function names in definitions and calls.
     */
    Always: 3, "3": "Always",
});

/**
 * @param {string} code
 * @param {Config} config
 * @param {Range | null | undefined} range
 * @param {OutputVerification} verify_output
 * @returns {string}
 */
export function formatCode(code, config, range, verify_output) {
    let deferred5_0;
    let deferred5_1;
    try {
        const ptr0 = passStringToWasm0(code, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(config, Config);
        var ptr1 = config.__destroy_into_raw();
        let ptr2 = 0;
        if (!isLikeNone(range)) {
            _assertClass(range, Range);
            ptr2 = range.__destroy_into_raw();
        }
        const ret = wasm.formatCode(ptr0, len0, ptr1, ptr2, verify_output);
        var ptr4 = ret[0];
        var len4 = ret[1];
        if (ret[3]) {
            ptr4 = 0; len4 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred5_0 = ptr4;
        deferred5_1 = len4;
        return getStringFromWasm0(ptr4, len4);
    } finally {
        wasm.__wbindgen_free(deferred5_0, deferred5_1, 1);
    }
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./stylua_lib_bg.js": import0,
    };
}

const ConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_config_free(ptr >>> 0, 1));
const RangeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_range_free(ptr >>> 0, 1));
const SortRequiresConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sortrequiresconfig_free(ptr >>> 0, 1));

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

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
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
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

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('stylua_lib_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
