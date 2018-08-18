#![feature(use_extern_macros)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::collections::HashSet;

#[derive(PartialEq, Clone)]
enum Player {
    NoOne,
    P1,
    P2
}

#[derive(PartialEq)]
enum MoveType {
    Take,
    Jump,
    Illegal
}

struct Move {
    moveType: MoveType,
    from: Position,
    to: Position
}

#[derive(Clone)]
struct Board {
    squares: Vec<Vec<Player>>,
    edge_size: usize
}

#[derive(Clone)]
struct BoardAndPoints {
    board: Board,
    points: i32
}

#[derive(Clone)]
struct BoardAndPointsAndPossibleMoves {
    boardAndPoints: BoardAndPoints,
    player: Player,
    takes: Vec<(Position, Position)>,
    jumps: Vec<(Position, Position)>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Position {
    row: usize,
    col: usize
}

struct MoveResult {
    theMove: Move,
    startBoard: BoardAndPointsAndPossibleMoves,
    endBoard: BoardAndPoints
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

    // Reverse polarity
    fn other_player(&self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
            Player::NoOne => Player::NoOne
        }
    }
}

// Static
impl Board {
    // parse board string into board object
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

    // return true if the player can take that position
    fn can_take(&self, player: &Player, pos: &Position) -> Option<Move> {
        let edge_size = self.edge_size;
        let take_rows: Vec<usize> = {
            if pos.row == 0 { vec![0, 1] }
            else if pos.row == edge_size-1 { vec![edge_size-2, edge_size-1] }
            else { vec![pos.row-1, pos.row, pos.row + 1] }
        };
        let take_cols: Vec<usize> = {
            if pos.col == 0 { vec![0, 1] }
            else if pos.col == edge_size-1 { vec![edge_size-2, edge_size-1] }
            else { vec![pos.col-1, pos.col, pos.col + 1] }
        };

        for row in &take_rows {
            for col in &take_cols {
                if &self.squares[*row][*col] == player { 
                    return Some(Move {
                        moveType: MoveType::Take,
                        from: Position{row: *row, col: *col},
                        to: pos.clone()
                    })
                }
            }
        }
        None
    }

    // return a vec of all possible starting points for a jump to that position
    fn get_jumps(&self, player: &Player, pos: &Position) -> Vec<Position> {
        let mut ret: Vec<Position> = Vec::new();
        let edge_size = self.edge_size;
        // for each board square, could that square jump to the desired position
        for row in 0..self.edge_size {
            for col in 0..self.edge_size {
                let owner = &self.squares[row][col];
                if owner != player { continue; } // not if the player doesnt own it

                let row_delta = usize_abs_delta(row, pos.row);
                let col_delta = usize_abs_delta(col, pos.col);
                if (
                    (row_delta == 2 && col_delta <= 2) ||
                    (col_delta == 2 && row_delta <= 2)
                ) {
                    ret.push(Position{ row, col });
                }
            }
        }
        ret
    }

    // vector of all positions that are nonempty
    fn get_all_moves(&self, player: &Player) -> (Vec<(Position, Position)>, Vec<(Position, Position)>) {
        let mut takes: Vec<(Position, Position)> = Vec::new();
        let mut jumps: Vec<(Position, Position)> = Vec::new();
        for row in 0..self.edge_size {
            for col in 0..self.edge_size {
                let owner = &self.squares[row][col];
                if owner != &Player::NoOne {
                    continue;
                }
                let pos = Position{ row, col};
                match self.can_take(player, &pos) {
                    Some(m) => takes.push((m.from.clone(), m.to.clone())),
                    _ => {
                        let jump_pairs = self.get_jumps(player, &pos);
                        for from_pos in jump_pairs {
                            jumps.push((from_pos.clone(), pos.clone()));
                        }
                    }
                }
            }
        }
        (takes, jumps)
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
}

// Instance
impl BoardAndPoints {
    fn attach_moves(&self, player: &Player) -> BoardAndPointsAndPossibleMoves {
        let moves = self.board.get_all_moves(player);
        BoardAndPointsAndPossibleMoves {
            boardAndPoints: self.clone(),
            player: player.clone(),
            takes: moves.0,
            jumps:  moves.1
        }
    }

/*


    // Step 1: Square => Set of all moves and results of moving to that square
    // Given a moving player, board, and a square to move to, return a BoardAndPoints for the result of each move that moves into that square
    // i.e. 0-1 takes, 0-m jumps, empty vec if there's no way for that player to move into that square
    fn get_all_moves_to_square(&self, player: &Player, to: Position) -> Vec<BoardAndPoints> {
        let mut result: Vec<BoardAndPoints> = vec![];
        let occupying_player = &self.board.squares[to.row][to.col];
        // if someone already has that square, bail
        if occupying_player != &Player::NoOne { return result };

        // can we take it with an adjacent?
        // 

        result
    }*/

    // Step 1a: Square => particular move type into square => Results of that single move
    fn move_into_square(&self, player: &Player, is_opponent: bool, theMove: &Move) -> BoardAndPoints {
        // Assume that some other function has already determined that moving from that square into that square with the given movetype is valid

        // start with the 1 point for taking a new square, if this is a take
        let mut delta: i32 = {
            match theMove.moveType {
                MoveType::Take => 1,
                _ => 0
            }
        };
        let to: &Position = &theMove.to;
        let edge_size = self.board.edge_size;
        let mut new_board: Board = self.board.clone();

        // Take all the surrounding squares from the other player
        let take_rows: Vec<usize> = {
            if to.row == 0 { vec![0, 1] }
            else if to.row == edge_size-1 { vec![edge_size-2, edge_size-1] }
            else { vec![to.row-1, to.row, to.row + 1] }
        };
        let take_cols: Vec<usize> = {
            if to.col == 0 { vec![0, 1] }
            else if to.col == edge_size-1 { vec![edge_size-2, edge_size-1] }
            else { vec![to.col-1, to.col, to.col + 1] }
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
        if theMove.moveType == MoveType::Jump {
            let p = &mut new_board.squares[theMove.from.row][theMove.from.col];
            *p = Player::NoOne;
        }

        BoardAndPoints{
            board: new_board,
            points: self.points + delta
        }
    }
}

impl BoardAndPointsAndPossibleMoves {
    // Entry point
    fn find_best_move(&self, is_opponent: bool, levels_left: u8) -> MoveResult {
        let firstTake = (&self.takes[0]).clone();
        let theMove = Move {
            moveType: MoveType::Take,
            from: firstTake.0,
            to: firstTake.1
        };
        let startBoard: BoardAndPointsAndPossibleMoves = self.clone();
        let endBoard: BoardAndPoints = self.boardAndPoints.move_into_square(&self.player, false, &theMove);
        MoveResult{
            theMove,
            startBoard,
            endBoard
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
    let withMoves: BoardAndPointsAndPossibleMoves = start.attach_moves(&Player::P2);
    let moveResult: MoveResult = withMoves.find_best_move(false, 1);
    let theMove = moveResult.theMove;
    theMove.from.row.to_string() + "," + &theMove.from.col.to_string() + ">" + &theMove.to.row.to_string() + "," + &theMove.to.col.to_string()
}
