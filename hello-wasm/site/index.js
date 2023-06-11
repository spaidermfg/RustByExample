const js = import("./node_modules/@minfg67/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});
