#![feature(use_extern_macros)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone)]
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

    fn other_player(&self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
            Player::NoOne => Player::NoOne
        }
    }
}

#[derive(PartialEq)]
enum Move {
    Take,
    Jump,
    Illegal
}

#[derive(Clone)]
struct Board {
    squares: Vec<Vec<Player>>,
    edge_size: usize
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
        let edge_size = squares.len();
        let board = Board {
            squares,
            edge_size
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
/*
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
*/
    fn move_into_square(player: &Player, is_opponent: bool, board_and_points: &BoardAndPoints, from_row: usize, from_col: usize, to_row: usize, to_col: usize, move_type: &Move) -> BoardAndPoints {
        // Assume that some other function has already determined that moving from that square into that square with the given movetype is valid

        // start with the 1 point for taking a new square, if this is a take
        let mut delta: i32 = {
            if move_type == &Move::Take { 1 }
            else { 0 }
        };
        let edge_size = board_and_points.board.edge_size;
        let mut new_board: Board = board_and_points.board.clone();

        // Take all the surrounding squares from the other player
        let take_rows: Vec<usize> = {
            if to_row == 0 { vec![0, 1] }
            else if to_row == edge_size-1 { vec![edge_size-2, edge_size-1] }
            else { vec![to_row-1, to_row, to_row + 1] }
        };
        let take_cols: Vec<usize> = {
            if to_col == 0 { vec![0, 1] }
            else if to_col == edge_size-1 { vec![edge_size-2, edge_size-1] }
            else { vec![to_col-1, to_col, to_col + 1] }
        };
        for row in &take_rows {
            for col in &take_cols {
                let squares = &mut new_board.squares;
                let row = &mut squares[*row];
                let p = &mut row[*col];
                if p == &player.other_player() {
                    *p = player.clone();
                    delta += 2; // each square we steal from the other player is worth +2 net score
                }
            }
        }

        // If this is an opponent turn, reverse the delta
        if is_opponent {
            delta *= -1;
        }

        // If this is a jump, free the `from` square
        if move_type == &Move::Jump {
            let p = &mut new_board.squares[from_row][from_col];
            *p = Player::NoOne;
        }

        BoardAndPoints{
            board: new_board,
            points: board_and_points.points + delta
        }
    }


    // Given a moving player, board, and a square to move to, return a BoardAndPoints for the result of each move that moves into that square
    // i.e. 0-1 takes, 0-m jumps, empty vec if there's no way for that player to move into that square
    fn get_all_moves_to_square(player: &Player, board_and_points: &BoardAndPoints, row: usize, col: usize) -> Vec<BoardAndPoints> {
        let mut result: Vec<BoardAndPoints> = vec![];
        let occupying_player = &board_and_points.board.squares[row][col];
        // if someone already has that square, bail
        if occupying_player != &Player::NoOne { return result };


        result
    }

    fn find_best_move(player: &Player, is_opponent: bool, board_and_points: BoardAndPoints, levels_left: u8) {
        // for each square, get vector of possible moves that result in gaining that square (0-1 takes, 0-m jumps)
        // for each move, map to BoardAndPoints node
        // for each node, recurse
        let squares = board_and_points.board.squares;
        for row in squares {
            for col in row {

            }
        }
    }
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
