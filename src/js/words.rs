use rand::Rng;
use wasm_bindgen::prelude::*;

const LETTERS: &str = "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZzAaBbCcDdEe";
const NUMBERS: &str = "123456789123456789123456789123456789123456789123456789123456789123456789";
const LETTERS_AND_NUMBERS: &str =
    "AaBb9CcDdEe8FfGgHh7IiJjKk6LlMmN5nOoPpQ4qRrSsT3tUuVvWwX2xYyZzAa1BbCcDdEe.";
const SYMBOLS: &str =
    "!@#$%^&*()-_+={}[];:,<.>/?|`~!@#$%^&*()-_+={}[];:,<.>/?|`~!@#$%^&*()-_+={}[];:,<.>/?|`~";
const KITCHEN_SINK: &str =
    "~q>fJ?487%t169\"32kH/Z^y-Q5s|L=O,eR0jG&W(MV)B!`K)I}r:P@d'$cN?b>x<w=v[u^";

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

#[wasm_bindgen]
#[doc = "gens a randomized email address @kintro.co"]
pub fn email_jenner(length: u32) -> String {
    let domain_part = "@kintro.co";
    let mut local_part = String::new();

    for i in 1..=length {
        local_part.push_str(&rand_char(LETTERS_AND_NUMBERS));
    }

    format!("{}{}", local_part, domain_part)
}

#[wasm_bindgen]
#[doc = "gens a randomized username(s) taking number of names as the arg"]
pub fn username_jenner(number_of_names: u32) -> String {
    let mut full_name: Vec<String> = Vec::new();

    fn build_word(word_len: u32) -> String {
        let mut word = String::new();

        for i in 1..=word_len {
            word.push_str(&rand_char(LETTERS));
        }

        word
    }

    for i in 1..=number_of_names {
        let word_len = rand::thread_rng().gen_range(1..=10);

        full_name.push(build_word(word_len));

        if i != number_of_names {
            full_name.push(String::from(" "));
        }
    }

    full_name.join("")
}

#[wasm_bindgen]
#[doc = "returns a string of length `length` made up of letters"]
pub fn password_jenner(length: u32) -> String {
    let mut password = String::new();

    for i in 1..=length {
        password.push_str(&rand_char(KITCHEN_SINK));
    }

    password
}

#[wasm_bindgen]
#[doc = "takes in sample &str and returns a random char from it, used in other helpers"]
pub fn rand_char(sample: &str) -> String {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..sample.len());
    let rnd_char = sample.chars().nth(i).unwrap();

    rnd_char.to_string()
}

#[wasm_bindgen]
#[doc = "returns a string of length `length` made up of letters"]
pub fn paragraph(length: u32) -> String {
    let mut paragraph: Vec<String> = Vec::new();

    for i in 1..=length {
        let x = rand::thread_rng().gen_range(1..=10);

        match x {
            3 | 7 => paragraph.push(String::from(" ")),
            _ => paragraph.push(rand_char(LETTERS)),
        }
    }

    paragraph.join("")
}

#[wasm_bindgen]
#[doc = "returns a string of length `length` made up of numbers"]
pub fn number_paragraph(length: u32) -> String {
    let mut paragraph: Vec<String> = Vec::new();

    for i in 1..=length {
        let x = rand::thread_rng().gen_range(1..=10);

        match x {
            3 | 7 => paragraph.push(String::from(" ")),
            _ => paragraph.push(rand_char(NUMBERS)),
        }
    }

    paragraph.join("")
}

#[wasm_bindgen]
#[doc = "returns a string of length `length` made up of letters & numbers"]
pub fn letter_and_number_paragraph(length: u32) -> String {
    let mut paragraph: Vec<String> = Vec::new();

    for i in 1..=length {
        let x = rand::thread_rng().gen_range(1..=10);

        match x {
            3 | 7 => paragraph.push(String::from(" ")),
            _ => paragraph.push(rand_char(LETTERS_AND_NUMBERS)),
        }
    }

    paragraph.join("")
}

#[wasm_bindgen]
#[doc = "returns a string of length `length` made up of symbols"]
pub fn symbol_paragraph(length: u32) -> String {
    let mut paragraph: Vec<String> = Vec::new();

    for i in 1..=length {
        let x = rand::thread_rng().gen_range(1..=10);

        match x {
            3 | 7 => paragraph.push(String::from(" ")),
            _ => paragraph.push(rand_char(SYMBOLS)),
        }
    }

    paragraph.join("")
}

#[wasm_bindgen]
#[doc = "returns a string of length `length` made up of symbols"]
pub fn kitchen_sink_paragraph(length: u32) -> String {
    let mut paragraph: Vec<String> = Vec::new();

    for i in 1..=length {
        let x = rand::thread_rng().gen_range(1..=10);

        match x {
            3 | 7 => paragraph.push(String::from(" ")),
            _ => paragraph.push(rand_char(KITCHEN_SINK)),
        }
    }

    paragraph.join("")
}
