WebAssembly.instantiateStreaming(fetch("bones.wasm")).then(wasmModule => {
    var add = wasmModule.instance.exports.add;
    console.log("add(2,3)", add(2,3));
});
