var wasmModule = null;
const importObject = {
    env: {
        consoleLog: function (ptr) {
            console.log("wasm says:", fromCString(ptr));
        }
    }
}

WebAssembly.instantiateStreaming(fetch("bones.wasm"), importObject).then(wm => {
    wasmModule = wm;
    const add = wasmModule.instance.exports.add;

    console.log("add(2,3)", add(2, 3));
    console.log("write('aa','bb')", write('aa', 'bb'));
    console.log("read('aa')", read('aa'));
});

function write(key, value) {
    return !!wasmModule.instance.exports.write(...[key, value].map(toCString));
}

function read(key) {
    return fromCString(wasmModule.instance.exports.read(toCString(key)));
}

function toCString(s) {
    const stringSize = s.length + 1;
    const ptr = wasmModule.instance.exports.alloc_bytes(stringSize);
    const buff = new Uint8Array(wasmModule.instance.exports.memory.buffer, ptr, stringSize);
    for (let i = 0; i < s.length; i++)
        buff[i] = s.charCodeAt(i);
    buff[s.length] = 0;
    return ptr;
}

function fromCString(ptr) {
    const buff = new Uint8Array(wasmModule.instance.exports.memory.buffer, ptr);
    let s = "";
    while (buff[s.length] != 0) {
        s += String.fromCharCode(buff[s.length]);
    }
    wasmModule.instance.exports.dealloc_cstring(ptr);
    return s;
}
