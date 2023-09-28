use rand::Rng;
use wasm_bindgen::prelude::*;

pub fn random_long_int(beg: i32, end: i32) -> i32 {
    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(beg..=end);

    random_number
}
