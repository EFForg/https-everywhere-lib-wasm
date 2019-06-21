(function() {
    const __exports = {};
    let wasm;

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

    let stack_pointer = 32;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }

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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

__exports.__wbg_new_acdbe9c25dc35c37 = function() {
    return addHeapObject(new Array());
};

__exports.__wbg_from_66ee95be4fef51b2 = function(arg0) {
    return addHeapObject(Array.from(getObject(arg0)));
};

__exports.__wbg_isArray_1ac2d66dd819d58a = function(arg0) {
    return Array.isArray(getObject(arg0));
};

__exports.__wbg_length_2a2c78eb761c69a1 = function(arg0) {
    return getObject(arg0).length;
};

__exports.__wbg_push_60b55c9bdc824202 = function(arg0, arg1) {
    return getObject(arg0).push(getObject(arg1));
};

__exports.__wbg_values_e8a26a3eb612c7c3 = function(arg0) {
    return addHeapObject(getObject(arg0).values());
};

__exports.__wbg_valueOf_c4ec2eaaa602f4f8 = function(arg0) {
    return getObject(arg0).valueOf();
};

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

__exports.__wbg_next_1dc1c12b3aad066e = function(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).next());
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__wbg_done_8b0657c71869dd76 = function(arg0) {
    return getObject(arg0).done;
};

__exports.__wbg_value_efc53c71db10c238 = function(arg0) {
    return addHeapObject(getObject(arg0).value);
};

__exports.__wbg_new_68180085d411e1be = function() {
    return addHeapObject(new Object());
};

__exports.__wbg_add_13b2529fbab4a8fd = function(arg0, arg1) {
    return addHeapObject(getObject(arg0).add(getObject(arg1)));
};

__exports.__wbg_new_d8ff5ee3f007cfae = function(arg0) {
    return addHeapObject(new Set(getObject(arg0)));
};

__exports.__wbg_get_48d637c66043532c = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(Reflect.get(getObject(arg0), getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__wbg_set_8866dbb36cf947cb = function(arg0, arg1, arg2, exnptr) {
    try {
        return Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    } catch (e) {
        handleError(exnptr, e);
    }
};

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

__exports.__wbindgen_string_new = function(p, l) { return addHeapObject(getStringFromWasm(p, l)); };

__exports.__wbindgen_is_null = function(i) { return getObject(i) === null ? 1 : 0; };

__exports.__wbindgen_is_undefined = function(i) { return getObject(i) === undefined ? 1 : 0; };

__exports.__wbindgen_boolean_get = function(i) {
    let v = getObject(i);
    return typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
};

__exports.__wbindgen_is_object = function(i) {
    const val = getObject(i);
    return typeof(val) === 'object' && val !== null ? 1 : 0;
};

__exports.__wbindgen_is_string = function(i) { return typeof(getObject(i)) === 'string' ? 1 : 0; };

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

__exports.__wbindgen_string_get = function(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string') return 0;
    const ptr = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
    return ptr;
};

__exports.__wbindgen_debug_string = function(i, len_ptr) {
    const debug_str =
    val => {
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
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
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
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
};

__exports.__wbindgen_jsval_eq = function(a, b) { return getObject(a) === getObject(b) ? 1 : 0; };

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

function freeJsRuleSets(ptr) {

    wasm.__wbg_jsrulesets_free(ptr);
}
/**
* A newtype for rulesets, wrapping all the JS functionality
*/
class JsRuleSets {

    static __wrap(ptr) {
        const obj = Object.create(JsRuleSets.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeJsRuleSets(ptr);
    }

    /**
    * Returns a new JsRulesets struct
    * @returns {JsRuleSets}
    */
    static new() {
        return JsRuleSets.__wrap(wasm.jsrulesets_new());
    }
    /**
    * Returns the number of targets in the current JsRuleSets struct as a `usize`
    * @returns {number}
    */
    count_targets() {
        return wasm.jsrulesets_count_targets(this.ptr) >>> 0;
    }
    /**
    * Construct and add new rulesets given a JS array of values
    *
    * # Arguments
    *
    * * `array` - A JS Array object of rulesets
    * * `enable_mixed_rulesets` - A JS Boolean indicating whether rulesets which trigger mixed
    * content blocking should be enabled
    * * `rule_active_states` - A JS object which lets us know whether rulesets have been disabled
    * or enabled
    * * `scope` - An optional JS string which indicates the scope of the current batch of
    * rulesets being added (see the [ruleset update channels](https://github.com/EFForg/https-everywhere/blob/master/docs/en_US/ruleset-update-channels.md) documentation)
    * @param {any} array
    * @param {any} enable_mixed_rulesets
    * @param {any} rule_active_states
    * @param {any} scope
    * @returns {void}
    */
    add_all_from_js_array(array, enable_mixed_rulesets, rule_active_states, scope) {
        try {
            return wasm.jsrulesets_add_all_from_js_array(this.ptr, addBorrowedObject(array), addBorrowedObject(enable_mixed_rulesets), addBorrowedObject(rule_active_states), addBorrowedObject(scope));

        } finally {
            heap[stack_pointer++] = undefined;
            heap[stack_pointer++] = undefined;
            heap[stack_pointer++] = undefined;
            heap[stack_pointer++] = undefined;

        }

    }
    /**
    * Remove a RuleSet from the RuleSets struct
    * @param {any} ruleset_jsval
    * @returns {void}
    */
    remove_ruleset(ruleset_jsval) {
        try {
            return wasm.jsrulesets_remove_ruleset(this.ptr, addBorrowedObject(ruleset_jsval));

        } finally {
            heap[stack_pointer++] = undefined;

        }

    }
    /**
    * Return a JS set of rulesets that apply to the given host
    *
    * # Arguments
    *
    * * `host` - A JS string which indicates the host to search for potentially applicable rulesets
    * @param {any} host
    * @returns {any}
    */
    potentially_applicable(host) {
        try {
            return takeObject(wasm.jsrulesets_potentially_applicable(this.ptr, addBorrowedObject(host)));

        } finally {
            heap[stack_pointer++] = undefined;

        }

    }
}
__exports.JsRuleSets = JsRuleSets;

__exports.__wbindgen_object_drop_ref = function(i) { dropObject(i); };

function init(module) {
    let result;
    const imports = { './https_everywhere_lib_wasm': __exports };

    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

self.wasm_bindgen = Object.assign(init, __exports);

})();
