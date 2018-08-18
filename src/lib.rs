#![feature(use_extern_macros)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(PartialEq)]
enum Player {
    NoOne,
    P1,
    P2
}

// Static
impl Player {
    // parse a string as a player
    fn parse_string(s: &str) -> Player {
        if s == "2" {Player::P2}
        else if s == "1" {Player::P1}
        else {Player::NoOne}
    }
}

// Instance
impl Player {
    // how many points is a square for this player worth
    fn get_points(&self) -> i32 {
        match self {
            Player::P1 => 1,
            Player::P2 => -1,
            Player::NoOne => 0
        }
    }
}

enum Move {
    Take,
    Jump,
    Illegal
}

struct Board {
    squares: Vec<Vec<Player>>
}

struct BoardAndPoints {
    board: Board,
    points: i32
}

// Static
impl Board {
    fn parse_board(board_string: &str) -> BoardAndPoints {
        let rows: Vec<&str> = board_string.split(":").collect();
        let squares: Vec<Vec<Player>> = rows.into_iter().map(|row| {
            row.chars().map(|x| Player::parse_string(&x.to_string())).collect()
        }).collect();
        let board = Board {
            squares
        };
        let points = board.get_points();
        BoardAndPoints {
            board,
            points
        }
    }
}

// Instance
impl Board {
    // What is the score for this board
    fn get_points(&self) -> i32 {
        fn get_row_points(row: &Vec<Player>) -> i32 {
            row.into_iter().map(|p| p.get_points()).fold(0, |agg, x| agg + x)
        }
        let squares: &Vec<Vec<Player>> = &self.squares;
        let row_sums: Vec<i32> = squares.into_iter().map(|row| get_row_points(&row)).collect();
        row_sums.into_iter().fold(0, |agg, x| agg + x)
    }

    // given a board and a desire to move, what kind of move would it be
    fn move_type(&self, p: Player, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> Move {
        let player_on_to: &Player = &self.squares[to_row][to_col];
        if player_on_to != &Player::NoOne { return Move::Illegal; }
        let row_delta = usize_abs_delta(from_row, to_row);
        let col_delta = usize_abs_delta(from_col, to_col);
        if row_delta <=1 && col_delta <= 1 {
            Move::Take
        } else if row_delta <= 2 && col_delta <= 2 {
            Move::Jump
        } else {
            Move::Illegal
        }
    }

  /*  fn execute_move(&self, p: Player, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> Option<BoardAndPoints> {

    }*/
}

fn usize_abs_delta(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[wasm_bindgen]
pub fn greet(board_string: &str) -> String {
    let start: BoardAndPoints = Board::parse_board(board_string);
    "Hello, ".to_owned() + board_string + "!" + &start.points.to_string()
}
