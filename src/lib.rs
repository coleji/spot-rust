#![feature(use_extern_macros)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
/*
struct Board {
    squares: Vec<Vec<String>>
}

fn parse_board(board_string: &str) -> Board {
    let rows: Vec<&str> = board_string.split(":");
    let squares = rows.into_iter().map(|row: &str | row.split("")).collect();
    Board{
        squares
    }
}
*/
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    "Hello, ".to_owned() + name + "!"
}
