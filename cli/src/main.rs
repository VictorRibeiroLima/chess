use std::{
    io::{self, Write},
    str::FromStr,
};

use engine::{board::Board, piece::position::Position};

fn main() {
    println!("Welcome to Rust Chess!");
    println!("Please enter a move in the format: 'a2 a3'");
    let mut board = Board::new();
    println!("{}", board);
    while board.get_winner().is_none() {
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

    let winner = board.get_winner().unwrap();

    println!("{} wins!", winner);
}

fn get_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
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
