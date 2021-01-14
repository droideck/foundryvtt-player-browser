use wasm_bindgen::prelude::*;

//       
// TODO: 1. React waits for POST from the server
//       2. Rust parses and calculates the data to be printed
//       3. React draws the interface and continues to wait for another request
#[wasm_bindgen]
pub fn print_string() -> String {
    let foo_bar = "World";
    return format!("Hello {}", foo_bar);
}

