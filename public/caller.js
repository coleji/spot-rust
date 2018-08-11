function callRust() {
  console.log("in the caller");
  wasm_bindgen("./spot_rust_bg.wasm").then(() => wasm_bindgen.greet('Jon'))
}
