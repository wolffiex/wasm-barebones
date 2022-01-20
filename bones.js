var wasmModule = null;
const importObject = {
    env: {
        consoleLog: function (p) {
            console.log("wasm says:", fromCString(p));
        }
    }
}

WebAssembly.instantiateStreaming(fetch("bones.wasm"), importObject).then(m => {
    wasmModule = m;
    const add = wasmModule.instance.exports.add;

    console.log("add(2,3)", add(2, 3));
});

function write(key, val) {
    wasmModule.instance.exports.write(toCString(key), val);
}

function toCString(s) {
    const stringSize = s.length + 1;
    const p = wasmModule.instance.exports.alloc_cstring(stringSize);
    const m = new Uint8Array(wasmModule.instance.exports.memory.buffer, p, stringSize);
    for (let i = 0; i < s.length; i++)
        m[i] = s.charCodeAt(i);
    m[s.length] = 0;
    return p;
}

function fromCString(ptr) {
    const m = new Uint8Array(wasmModule.instance.exports.memory.buffer, ptr);
    let s = "";
    while (m[s.length] != 0) {
        s += String.fromCharCode(m[s.length]);
    }
    wasmModule.instance.exports.dealloc_cstring(ptr);
    return s;
}
