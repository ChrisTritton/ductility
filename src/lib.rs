use rand::Rng;
use wasm_bindgen::prelude::*;

extern "C" {
    fn alert(s: &str);
}

pub mod js;
pub mod rs;
