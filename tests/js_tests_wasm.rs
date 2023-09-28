#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use ductility::js::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn print() {
    println!("Cthulhu Fhtagn!!!");
}

// #[wasm_bindgen_test]
// fn lorem() {
//     let v1 = lorem("short");
//     println!("\nThe short lorem paragraph is: \n{:?}", v1);

//     let v2 = lorem("medium");
//     println!("\nThe medium lorem paragraph is: \n{:?}", v2);

//     let v3 = lorem("long");
//     println!("\nThe long lorem paragraph is: \n{:?}", v3);

//     assert_eq!(v1.len(), 276);
//     assert_eq!(v2.len(), 601);
//     assert_eq!(v3.len(), 1031);
// }

// #[wasm_bindgen_test]
// #[doc = "paragraph returned is of correct length"]
// fn paragraph() {
//     let s1 = paragraph(101);
//     println!("\nParagraph with 101 characters is: \n{:?}", s1);

//     let s2 = paragraph(666);
//     println!("\nParagraph with devilish characters is: \n{:?}", s2);

//     let s3 = paragraph(1331);
//     println!("\nParagraph with 1331 characters is: \n{:?}", s3);

//     assert_eq!(s1.len(), 101);
//     assert_eq!(s2.len(), 666);
//     assert_eq!(s3.len(), 1331);
// }

// #[wasm_bindgen_test]
// fn number_paragraph() {
//     let s1 = number_paragraph(100);
//     println!("\nParagraph with 100 characters is: \n{:?}", s1);

//     let s2 = number_paragraph(500);
//     println!("\nParagraph with 500 characters is: \n{:?}", s2);

//     let s3 = number_paragraph(1000);
//     println!("\nParagraph with 1000 characters is: \n{:?}", s3);

//     assert_eq!(s1.len(), 100);
//     assert_eq!(s2.len(), 500);
//     assert_eq!(s3.len(), 1000);
// }

// #[wasm_bindgen_test]
// fn letter_and_number_paragraph() {
//     let s1 = letter_and_number_paragraph(100);
//     println!("\nParagraph with 100 characters is: \n{:?}", s1);

//     let s2 = letter_and_number_paragraph(500);
//     println!("\nParagraph with 500 characters is: \n{:?}", s2);

//     let s3 = letter_and_number_paragraph(1000);
//     println!("\nParagraph with 1000 characters is: \n{:?}", s3);

//     assert_eq!(s1.len(), 100);
//     assert_eq!(s2.len(), 500);
//     assert_eq!(s3.len(), 1000);
// }

// #[wasm_bindgen_test]
// fn symbol_paragraph() {
//     let s1 = symbol_paragraph(100);
//     println!("\nParagraph with 100 characters is: \n{:?}", s1);

//     let s2 = symbol_paragraph(500);
//     println!("\nParagraph with 500 characters is: \n{:?}", s2);

//     let s3 = symbol_paragraph(1000);
//     println!("\nParagraph with 1000 characters is: \n{:?}", s3);

//     assert_eq!(s1.len(), 100);
//     assert_eq!(s2.len(), 500);
//     assert_eq!(s3.len(), 1000);
// }

// #[wasm_bindgen_test]
// fn kitchen_sink_paragraph() {
//     let s1 = kitchen_sink_paragraph(100);
//     println!("\nParagraph with 100 characters is: \n{:?}", s1);

//     let s2 = kitchen_sink_paragraph(500);
//     println!("\nParagraph with 500 characters is: \n{:?}", s2);

//     let s3 = kitchen_sink_paragraph(1000);
//     println!("\nParagraph with 1000 characters is: \n{:?}", s3);

//     assert_eq!(s1.len(), 100);
//     assert_eq!(s2.len(), 500);
//     assert_eq!(s3.len(), 1000);
// }
