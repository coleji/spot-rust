#![feature(use_extern_macros)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

enum Player {
    NoOne,
    P1,
    P2
}

fn get_player(s: &str) -> Player {
    if s == "2" {Player::P2}
    else if s == "1" {Player::P1}
    else {Player::NoOne}
}

fn get_player_points(p: &Player) -> i32 {
    match p {
        Player::P1 => 1,
        Player::P2 => -1,
        Player::NoOne => 0
    }
}

struct Board {
    squares: Vec<Vec<Player>>
}

struct BoardAndPoints {
    board: Board,
    points: i32
}

fn get_row_points(row: &Vec<Player>) -> i32 {
    row.into_iter().map(|p| get_player_points(&p)).fold(0, |agg, x| agg + x)
}

fn get_board_points(board: &Board) -> i32 {
    let squares: &Vec<Vec<Player>> = &board.squares;
    let row_sums: Vec<i32> = squares.into_iter().map(|row| get_row_points(&row)).collect();
    row_sums.into_iter().fold(0, |agg, x| agg + x)
}

fn parse_board(board_string: &str) -> BoardAndPoints {
    let rows: Vec<&str> = board_string.split(":").collect();
    let squares: Vec<Vec<Player>> = rows.into_iter().map(|row| {
        row.chars().map(|x| get_player(&x.to_string())).collect()
    }).collect();
    let board = Board {
        squares
    };
    let points = get_board_points(&board);
    BoardAndPoints {
        board,
        points: points
    }
}

#[wasm_bindgen]
pub fn greet(board_string: &str) -> String {
    let start: BoardAndPoints = parse_board(board_string);
    "Hello, ".to_owned() + board_string + "!" + &start.points.to_string()
}
