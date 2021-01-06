use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn impress_me(i: u32, j: u32) -> u32 {
       i * j
}
