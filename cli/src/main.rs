use std::{
    io::{self, Write},
    str::FromStr,
};

use engine::{
    board::{Board, GameState},
    piece::{position::Position, ChessPiece, Color},
};

fn main() {
    println!("Welcome to Rust Chess!");
    println!("Please enter a move in the format: 'a2 a3'");
    let mut board = Board::new();
    println!("{}", board);
    loop {
        match board.get_state() {
            GameState::Draw => {
                println!("Draw!");
                break;
            }
            GameState::Winner(color) => {
                println!("{} wins!", color);
                break;
            }
            GameState::WaitingPromotion(color, _) => match color {
                Color::White => promote_piece(&mut board),
                Color::Black => {
                    let choice = ai::make_promotion(&board);
                    board.promote(choice).unwrap();
                }
            },
            _ => {}
        };
        let turn = board.get_turn();

        match turn {
            Color::White => move_piece(&mut board),
            Color::Black => {
                let (from, to) = ai::make_move(&board);
                board.move_piece(from, to).unwrap();
            }
        }

        println!("{}", board);
    }
}

fn promote_piece(board: &mut Board) {
    println!("Promote a piece");
    println!("Options: Q, R, B, K");
    let input = get_input();
    let turn = board.get_turn();

    let piece = match input.trim() {
        "Q" => ChessPiece::create_queen(turn),
        "R" => ChessPiece::create_rook(turn),
        "B" => ChessPiece::create_bishop(turn),
        "K" => ChessPiece::create_knight(turn),
        _ => {
            println!("Invalid input");
            return;
        }
    };
    board.promote(piece).unwrap();
}

fn move_piece(board: &mut Board) {
    let input = get_input();
    let moves: Vec<&str> = input.trim().split(" ").collect();
    if moves.len() != 2 {
        println!("Invalid input");
        return;
    }
    let moves = input_to_moves(moves);
    let (from, to) = match moves {
        Some(moves) => moves,
        None => {
            println!("Invalid input");
            return;
        }
    };
    let moved = board.move_piece(from, to);
    match moved {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
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
