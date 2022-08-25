//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn new_minesweeper() {
    let ms = MineSweeper::new(2, 2, 2);
    let covered_with_mine = 10;
    assert_eq!(
        ms.mine_vec
            .iter()
            .filter(|&&item| item == covered_with_mine)
            .count(),
        2
    )
}
