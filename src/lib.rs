extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone)]
enum Player {
	NoOne,
	P1,
	P2,
}

#[derive(PartialEq, Clone)]
enum MoveType {
	Take,
	Jump,
}

#[derive(Clone)]
struct Move {
	move_type: MoveType,
	from: Position,
	to: Position,
}

#[derive(Clone)]
struct Board {
	squares: Vec<Vec<Player>>,
	edge_size: usize,
}

#[derive(Clone)]
struct BoardAndPoints {
	board: Board,
	points: i32,
}

#[derive(Clone)]
struct BoardAndPointsAndPossibleMoves {
	board_and_points: BoardAndPoints,
	player: Player,
	takes: Vec<Move>,
	jumps: Vec<Move>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Position {
	row: usize,
	col: usize,
}

#[derive(Clone)]
struct MoveResult {
	the_move: Move,
	start_board: BoardAndPointsAndPossibleMoves,
	end_board: BoardAndPoints,
	total_node_value: i32,
}

// Static
impl Player {
	// parse a string as a player
	fn parse_string(s: &str) -> Player {
		if s == "2" {
			Player::P2
		} else if s == "1" {
			Player::P1
		} else {
			Player::NoOne
		}
	}
}

// Instance
impl Player {
	// how many points is a square for this player worth
	fn get_points(&self) -> i32 {
		match self {
			Player::P1 => 1,
			Player::P2 => -1,
			Player::NoOne => 0,
		}
	}

	// Reverse polarity
	fn other_player(&self) -> Player {
		match self {
			Player::P1 => Player::P2,
			Player::P2 => Player::P1,
			Player::NoOne => Player::NoOne,
		}
	}

	fn score_is_better(&self, new_score: i32, old_score: i32) -> bool {
		match self {
			Player::P1 => new_score > old_score,
			Player::P2 => old_score > new_score,
			Player::NoOne => true,
		}
	}

	fn to_string(&self) -> &str {
		match self {
			Player::P1 => "1",
			Player::P2 => "2",
			Player::NoOne => "0",
		}
	}
}

// Static
impl Board {
	// parse board string into board object
	fn parse_board(board_string: &str) -> BoardAndPoints {
		let rows: Vec<&str> = board_string.split(":").collect();
		let squares: Vec<Vec<Player>> = rows
			.into_iter()
			.map(|row| {
				row.chars()
					.map(|x| Player::parse_string(&x.to_string()))
					.collect()
			})
			.collect();
		let edge_size = squares.len();
		let board = Board { squares, edge_size };
		let points = board.get_points();
		BoardAndPoints { board, points }
	}

	fn starting_board(edge_size: usize) -> Board {
		fn who_starts(row: usize, col: usize, edge_size: usize) -> Player {
			let top_left = Player::P1;
			let top_right = top_left.other_player();
			let edge_minus_one = edge_size-1;
			if row == 0 && col == 0 { top_left }
			else if row == edge_minus_one && col == edge_minus_one { top_left }
			else if row == 0 && col == edge_minus_one { top_right }
			else if row == edge_minus_one && col == 0 { top_right }
			else { Player::NoOne }
		}

		let mut squares: Vec<Vec<Player>> = Vec::new();
		for row in 0..edge_size {
			let mut row_vec: Vec<Player> = Vec::new();
			for col in 0..edge_size {
				row_vec.push(who_starts(row, col, edge_size));
			}
			squares.push(row_vec);
		}
		Board {
			squares,
			edge_size
		}
	}
}

// Instance
impl Board {
	fn serialize_board(&self) -> String {
		let rows_vec: Vec<String> = (&self.squares).into_iter().map(|row| {
			let players_vec: Vec<String> = row.into_iter().map(|cell| cell.to_string().to_string()).collect();
			players_vec.join("")
		}).collect();
		rows_vec.join(":")
	}

	// What is the score for this board
	fn get_points(&self) -> i32 {
		fn get_row_points(row: &Vec<Player>) -> i32 {
			row.into_iter()
				.map(|p| p.get_points())
				.fold(0, |agg, x| agg + x)
		}
		let squares: &Vec<Vec<Player>> = &self.squares;
		let row_sums: Vec<i32> = squares
			.into_iter()
			.map(|row| get_row_points(&row))
			.collect();
		row_sums.into_iter().fold(0, |agg, x| agg + x)
	}

	// return Some(arbitrary `from` position) if the player can take that position, None otherwise
	fn can_take(&self, player: &Player, pos: &Position) -> Option<Move> {
		let edge_size = self.edge_size;
		let take_rows: Vec<usize> = {
			if pos.row == 0 {
				vec![0, 1]
			} else if pos.row == edge_size - 1 {
				vec![edge_size - 2, edge_size - 1]
			} else {
				vec![pos.row - 1, pos.row, pos.row + 1]
			}
		};
		let take_cols: Vec<usize> = {
			if pos.col == 0 {
				vec![0, 1]
			} else if pos.col == edge_size - 1 {
				vec![edge_size - 2, edge_size - 1]
			} else {
				vec![pos.col - 1, pos.col, pos.col + 1]
			}
		};

		for row in &take_rows {
			for col in &take_cols {
				if &self.squares[*row][*col] == player {
					return Some(Move {
						move_type: MoveType::Take,
						from: Position {
							row: *row,
							col: *col,
						},
						to: pos.clone(),
					});
				}
			}
		}
		None
	}

	// return a vec of all possible starting points for a jump to that position
	fn get_jumps(&self, player: &Player, pos: &Position) -> Vec<Position> {
		let mut ret: Vec<Position> = Vec::new();
		// for each board square, could that square jump to the desired position
		for row in 0..self.edge_size {
			for col in 0..self.edge_size {
				let owner = &self.squares[row][col];
				if owner != player {
					continue;
				} // not if the player doesnt own it

				let row_delta = usize_abs_delta(row, pos.row);
				let col_delta = usize_abs_delta(col, pos.col);
				if (row_delta == 2 && col_delta <= 2) || (col_delta == 2 && row_delta <= 2) {
					ret.push(Position { row, col });
				}
			}
		}
		ret
	}

	// vector of all takes and all jumps
	fn get_all_moves(&self, player: &Player) -> (Vec<Move>, Vec<Move>) {
		let mut takes: Vec<Move> = Vec::new();
		let mut jumps: Vec<Move> = Vec::new();
		for row in 0..self.edge_size {
			for col in 0..self.edge_size {
				let owner = &self.squares[row][col];
				if owner != &Player::NoOne {
					continue;
				}
				let pos = Position { row, col };
				match self.can_take(player, &pos) {
					Some(m) => takes.push(Move {
						move_type: MoveType::Take,
						from: m.from.clone(),
						to: m.to.clone(),
					}),
					_ => {
						let jump_pairs = self.get_jumps(player, &pos);
						for from_pos in jump_pairs {
							jumps.push(Move {
								move_type: MoveType::Jump,
								from: from_pos.clone(),
								to: pos.clone(),
							});
						}
					}
				}
			}
		}
		(takes, jumps)
	}
}

// Instance
impl BoardAndPoints {
	fn attach_moves(&self, player: &Player) -> BoardAndPointsAndPossibleMoves {
		let moves = self.board.get_all_moves(player);
		BoardAndPointsAndPossibleMoves {
			board_and_points: self.clone(),
			player: player.clone(),
			takes: moves.0,
			jumps: moves.1,
		}
	}

	// Square => particular move type into square => Results of that single move
	fn move_into_square(
		&self,
		is_p2: bool,
		the_move: &Move,
	) -> BoardAndPoints {
		let player = if is_p2 { Player::P2 } else { Player::P1 };
		// Assume that some other function has already determined that moving from that square into that square with the given movetype is valid

		// start with the 1 point for taking a new square, if this is a take
		let mut delta: i32 = {
			match the_move.move_type {
				MoveType::Take => 1,
				_ => 0,
			}
		};
		let to: &Position = &the_move.to;
		let edge_size = self.board.edge_size;
		let mut new_board: Board = self.board.clone();

		// Take all the surrounding squares from the other player
		let take_rows: Vec<usize> = {
			if to.row == 0 {
				vec![0, 1]
			} else if to.row == edge_size - 1 {
				vec![edge_size - 2, edge_size - 1]
			} else {
				vec![to.row - 1, to.row, to.row + 1]
			}
		};
		let take_cols: Vec<usize> = {
			if to.col == 0 {
				vec![0, 1]
			} else if to.col == edge_size - 1 {
				vec![edge_size - 2, edge_size - 1]
			} else {
				vec![to.col - 1, to.col, to.col + 1]
			}
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
		if is_p2 {
			delta *= -1;
		}

		// If this is a jump, free the `from` square
		if the_move.move_type == MoveType::Jump {
			let p = &mut new_board.squares[the_move.from.row][the_move.from.col];
			*p = Player::NoOne;
		}

		BoardAndPoints {
			board: new_board,
			points: self.points + delta,
		}
	}

	fn get_best_move(&self, is_p2: bool, levels_left: u8) -> (Option<Move>, i32) {
		if levels_left == 0 {
			(None, self.points)
		} else {
			let moving_player = if is_p2 {
				&Player::P2
			} else {
				&Player::P1
			};
			let board_with_moves = self.attach_moves(moving_player);
			let all_moves: Vec<Move> =
				[&board_with_moves.takes[..], &board_with_moves.jumps[..]].concat();
			let nodes: Vec<(Move, BoardAndPoints)> = all_moves
				.iter()
				.map(|the_move| {
					let resulting_board_w_points =
						self.move_into_square(is_p2, &the_move);
					(the_move.clone(), resulting_board_w_points)
				})
				.collect();

			let best_node = nodes
				.iter()
				.map(|node| {
					(&node.0, &node.1, node.1.get_best_move(!is_p2, levels_left - 1).1)
				})
				.fold(None, |best: Option<(&Move, &BoardAndPoints, i32)>, node| {
					match best {
						Some(b) => {
							if moving_player.score_is_better(node.1.points, b.1.points) {
								Some(node.clone())
							} else {
								Some(b)
							}
						}
						None => Some(node.clone())
					}	
				});

			match best_node {
				Some(t) => {
					(Some(t.0.clone()), t.2)
				},
				None => (None, self.points)
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
pub fn invert_player(player_string: &str) -> String {
	let start_player = Player::parse_string(player_string);
	let new_player = start_player.other_player();
	new_player.to_string().to_string()
}

#[wasm_bindgen]
pub fn calc_next_move(board_string: &str) -> String {
	let ai_depth = 3u8;
	log_many("ai depth: ", &ai_depth.to_string());
	let start: BoardAndPoints = Board::parse_board(board_string);
	log_many("current board value: ", &start.points.to_string());
	let node_value = start.get_best_move(true, ai_depth);
	let the_move = node_value.0;

	match the_move {
		Some(m) => {
			m.from.row.to_string()
				+ "," + &m.from.col.to_string()
				+ ">" + &m.to.row.to_string()
				+ "," + &m.to.col.to_string()
		}
		None => "".to_string(),
	}
}

#[wasm_bindgen]
pub fn new_board(edge_size: usize) -> String {
	let board = Board::starting_board(edge_size);
	board.serialize_board()
}

#[wasm_bindgen]
extern "C" {
	// Use `js_namespace` here to bind `console.log(..)` instead of just
	// `log(..)`
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	// The `console.log` is quite polymorphic, so we can bind it with multiple
	// signatures. Note that we need to use `js_name` to ensure we always call
	// `log` in JS.
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_u32(a: u32);

	// Multiple arguments too!
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_many(a: &str, b: &str);
}
