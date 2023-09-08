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

    let board = Board::mock(pieces, Color::Black, None);

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

    let board = Board::mock(pieces, Color::White, None);

    let from = Position::from_str("c2").unwrap();
    let to = Position::from_str("c4").unwrap();

    let pawn = board.get_piece_at(&from).unwrap();

    assert!(!pawn.can_move(from, to, &board));
}

#[test]
fn test_cant_make_a_movement_that_dont_remove_check() {
    let first_white_row = [
        Some(ChessPiece::create_rook(Color::White)),
        Some(ChessPiece::create_knight(Color::White)),
        Some(ChessPiece::create_bishop(Color::White)),
        None,
        Some(ChessPiece::create_king(Color::White)),
        Some(ChessPiece::create_bishop(Color::White)),
        Some(ChessPiece::create_knight(Color::White)),
        Some(ChessPiece::create_rook(Color::White)),
    ];

    let white_second_row = [
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        None,
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
    ];

    let fourth_row = [
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_queen(Color::White)),
        None,
        None,
        None,
    ];

    let black_first_row = [
        Some(ChessPiece::create_rook(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        Some(ChessPiece::create_bishop(Color::Black)),
        Some(ChessPiece::create_queen(Color::Black)),
        Some(ChessPiece::create_king(Color::Black)),
        Some(ChessPiece::create_bishop(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        Some(ChessPiece::create_rook(Color::Black)),
    ];

    let black_second_row = [
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_white_row,
        white_second_row,
        [None; 8],
        fourth_row,
        [None; 8],
        [None; 8],
        black_second_row,
        black_first_row,
    ];

    let board = Board::mock(pieces, Color::Black, Some(Color::Black));

    let from = Position::from_str("a7").unwrap();
    let to = Position::from_str("a5").unwrap();

    let pawn = board.get_piece_at(&from).unwrap();

    assert!(!pawn.can_move(from, to, &board));
}

#[test]
fn test_can_make_a_movement_that_remove_check() {
    let first_white_row = [
        Some(ChessPiece::create_rook(Color::White)),
        Some(ChessPiece::create_knight(Color::White)),
        Some(ChessPiece::create_bishop(Color::White)),
        None,
        Some(ChessPiece::create_king(Color::White)),
        Some(ChessPiece::create_bishop(Color::White)),
        Some(ChessPiece::create_knight(Color::White)),
        Some(ChessPiece::create_rook(Color::White)),
    ];

    let white_second_row = [
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        None,
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
    ];

    let fourth_row = [
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_queen(Color::White)),
        None,
        None,
        None,
    ];

    let black_first_row = [
        Some(ChessPiece::create_rook(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        Some(ChessPiece::create_bishop(Color::Black)),
        Some(ChessPiece::create_queen(Color::Black)),
        Some(ChessPiece::create_king(Color::Black)),
        Some(ChessPiece::create_bishop(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        Some(ChessPiece::create_rook(Color::Black)),
    ];

    let black_second_row = [
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_white_row,
        white_second_row,
        [None; 8],
        fourth_row,
        [None; 8],
        [None; 8],
        black_second_row,
        black_first_row,
    ];

    let board = Board::mock(pieces, Color::Black, Some(Color::Black));

    let from = Position::from_str("e8").unwrap();
    let to = Position::from_str("d7").unwrap();

    let king = board.get_piece_at(&from).unwrap();

    assert!(king.can_move(from, to, &board));
}
