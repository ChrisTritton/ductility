mod utils;

use rand::Rng;
use wasm_bindgen::prelude::*;

extern "C" {
    fn alert(s: &str);
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

#[wasm_bindgen]
pub fn lorem(length: &str) -> String {
    match length {
            "short" =>  String::from(
                "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system,
                and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness."
            ),
            "medium" => String::from(
                "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, 
                and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself,
                because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there
                anyone who loves or pursues or desires to obtain pain of itself, because it is pain."
            ),
            "long" => String::from(
                "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, 
                and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself,
                because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there
                anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure
                him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has
                any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure?",
            ),
            _ => String::from("Please provide a valid length input."),
        }
}
