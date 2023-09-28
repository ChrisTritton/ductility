mod utils;

use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, this is the beginning of the ductility testing utility library");
}

#[wasm_bindgen]
pub fn hello() -> String {
    String::from("Hello, this is the beginning of the ductility testing utility library")
}

#[wasm_bindgen]
pub fn random_long_int(beg: i32, end: i32) -> i32 {
    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(beg..=end);

    random_number
}
