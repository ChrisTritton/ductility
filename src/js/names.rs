use rand::Rng;
use wasm_bindgen::prelude::*;

use crate::store::gaming::*;

/*
just return a random name if nothing is specified, it'll be luck of the draw whether it's
a firstname or fullname


*/
#[wasm_bindgen]
pub fn name_picker() -> String {
    //(industry: String, game: Option<String>, fullname: Option<bool>) -> String {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(1..=14);

    DIABLO_II[rand_num].to_string()
}
