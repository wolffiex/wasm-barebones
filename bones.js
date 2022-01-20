var wasmModule = null;
const importObject = {
    env: {
        consoleLog: function(p) {
            console.log("wasm says:", fromCString(p));
        }
    }
}

WebAssembly.instantiateStreaming(fetch("bones.wasm"), importObject).then(m => {
    wasmModule = m;
    var add = wasmModule.instance.exports.add;
    console.log("add(2,3)", add(2,3));
});

function fromCString(ptr) {
    var m = new Uint8Array(wasmModule.instance.exports.memory.buffer, ptr);
    var s = "";
    let i = 0;
    while (m[i] != 0) {
        s += String.fromCharCode(m[i++]);
    }
    wasmModule.instance.exports.dealloc_cstring(ptr);
    return s;
}
