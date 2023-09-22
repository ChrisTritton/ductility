mod utils;

use wasm_bindgen::prelude::*;
use rand::Rng;

extern "C" {
    fn alert(s: &str);
}


pub mod js {
    use super::*;

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
}


pub mod rs {
    use super::*;

    pub fn hello() -> String {
        String::from("Hello, this is the beginning of the ductility testing utility library")
    }

    pub fn random_long_int(beg: i32, end: i32) -> i32 {
        let mut rng = rand::thread_rng();

        let random_number = rng.gen_range(beg..=end);

        random_number
    }
}
