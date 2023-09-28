//! Test suite for the Web and headless browsers. The doctests should be
//! just small examples. Here we can test our hepers in more detail.
//! These tests will also test that they run in the browser.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use ductility;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn print_from_web() {
    println!("PRINT SOMETHIGN!!!");
    assert_eq!(1 + 1, 2);
}
