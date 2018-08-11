wasm_bindgen("./spot_rust_bg.wasm")

function callRust(s, resolve) {
  console.log("in the caller");
  window.setTimeout(() => resolve(wasm_bindgen.greet(s)), 1);
}
