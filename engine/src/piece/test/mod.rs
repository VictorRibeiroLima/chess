use std::str::FromStr;

use crate::{board::Board, piece::position::Position};

use super::{ChessPiece, Color};

#[test]
fn bishop_movement_test() {
    let first_white_row = [
        Some(ChessPiece::create_rook(Color::White)),
        Some(ChessPiece::create_knight(Color::White)),
        Some(ChessPiece::create_bishop(Color::White)),
        Some(ChessPiece::create_queen(Color::White)),
        Some(ChessPiece::create_king(Color::White)),
        Some(ChessPiece::create_bishop(Color::White)),
        Some(ChessPiece::create_knight(Color::White)),
        Some(ChessPiece::create_rook(Color::White)),
    ];

    let second_white_row = [
        None,
        None,
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        None,
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
    ];

    let first_black_row = [
        Some(ChessPiece::create_rook(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        None,
        Some(ChessPiece::create_queen(Color::Black)),
        Some(ChessPiece::create_king(Color::Black)),
        Some(ChessPiece::create_bishop(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        Some(ChessPiece::create_rook(Color::Black)),
    ];

    let second_black_row = [
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
    ];

    let third_row = [
        None,
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::White)),
        None,
        None,
    ];

    let forth_row = [
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_bishop(Color::Black)),
        None,
        None,
        None,
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_white_row,
        second_white_row,
        third_row,
        forth_row,
        [None; 8],
        [None; 8],
        second_black_row,
        first_black_row,
    ];

    let board = Board::mock(pieces, Color::Black);

    let from = Position::from_str("e4").unwrap();
    let to = Position::from_str("f3").unwrap();
    let bishop = board.get_piece_at(&from).unwrap();

    assert!(bishop.can_move(from, to, &board));
}

#[test]
fn initial_pawn_movement_test() {
    let board = Board::new();

    let from = Position::from_str("c2").unwrap();
    let to = Position::from_str("c3").unwrap();

    let pawn = board.get_piece_at(&from).unwrap();

    assert!(pawn.can_move(from, to, &board));
}

#[test]

fn initial_pawn_movement_2_test() {
    let board = Board::new();

    let from = Position::from_str("c2").unwrap();
    let to = Position::from_str("c4").unwrap();

    let pawn = board.get_piece_at(&from).unwrap();

    assert!(pawn.can_move(from, to, &board));
}

#[test]
fn initial_pawn_movement_2_test_blocked() {
    let first_white_row = [
        Some(ChessPiece::create_rook(Color::White)),
        Some(ChessPiece::create_knight(Color::White)),
        Some(ChessPiece::create_bishop(Color::White)),
        Some(ChessPiece::create_queen(Color::White)),
        Some(ChessPiece::create_king(Color::White)),
        Some(ChessPiece::create_bishop(Color::White)),
        Some(ChessPiece::create_knight(Color::White)),
        Some(ChessPiece::create_rook(Color::White)),
    ];

    let second_white_row = [
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
    ];

    let third_row = [
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        None,
        None,
        None,
        None,
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_white_row,
        second_white_row,
        third_row,
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
    ];

    let board = Board::mock(pieces, Color::White);

    let from = Position::from_str("c2").unwrap();
    let to = Position::from_str("c4").unwrap();

    let pawn = board.get_piece_at(&from).unwrap();

    assert!(!pawn.can_move(from, to, &board));
}
