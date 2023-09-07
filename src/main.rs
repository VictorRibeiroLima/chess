use std::str::FromStr;

use board::Board;

use crate::piece::position::Position;

mod board;
mod piece;

fn main() {
    println!("Welcome to Rust Chess!");
    let mut board = Board::new();
    println!("{}", board);
    loop {
        let input = get_input();
        let moves: Vec<&str> = input.trim().split(" ").collect();
        if moves.len() != 2 {
            println!("Invalid input");
            continue;
        }
        let moves = input_to_moves(moves);
        let (from, to) = match moves {
            Some(moves) => moves,
            None => {
                println!("Invalid input");
                continue;
            }
        };
        let moved = board.move_piece(from, to);
        if !moved {
            println!("Invalid move");
            continue;
        }
        println!("{}", board);
    }
}

fn get_input() -> String {
    println!("Please enter a move in the format: 'a2 a3'");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn input_to_moves(input: Vec<&str>) -> Option<(Position, Position)> {
    let from = Position::from_str(input[0]);
    let to = Position::from_str(input[1]);
    match (from, to) {
        (Ok(from), Ok(to)) => Some((from, to)),
        _ => None,
    }
}
