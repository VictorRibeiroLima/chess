use std::str::FromStr;

use crate::{
    board::Board,
    piece::{position::Position, ChessPiece, Color},
};

#[test]
fn test_should_create_check() {
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

    let third_row = [
        None,
        None,
        None,
        Some(ChessPiece::create_queen(Color::White)),
        None,
        None,
        None,
        None,
    ];

    let fourth_row = [
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
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
        third_row,
        fourth_row,
        [None; 8],
        [None; 8],
        black_second_row,
        black_first_row,
    ];

    let mut board = Board::mock(pieces, Color::White, None);

    let from = Position::from_str("d3").unwrap();
    let to = Position::from_str("e4").unwrap();

    assert!(board.move_piece(from, to));
    if let Some(check) = board.get_check() {
        assert_eq!(check, Color::Black);
    } else {
        panic!("Expected check");
    }
}

#[test]
fn test_should_create_check_2() {
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

    let third_row = [
        None,
        None,
        None,
        Some(ChessPiece::create_queen(Color::White)),
        None,
        None,
        None,
        None,
    ];

    let fourth_row = [
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        None,
        None,
        None,
    ];

    let black_first_row = [
        Some(ChessPiece::create_rook(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        Some(ChessPiece::create_bishop(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_king(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        Some(ChessPiece::create_rook(Color::Black)),
    ];

    let black_second_row = [
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_white_row,
        white_second_row,
        third_row,
        fourth_row,
        [None; 8],
        [None; 8],
        black_second_row,
        black_first_row,
    ];

    let mut board = Board::mock(pieces, Color::White, None);

    let from = Position::from_str("d3").unwrap();
    let to = Position::from_str("e4").unwrap();

    assert!(board.move_piece(from, to));
    if let Some(check) = board.get_check() {
        assert_eq!(check, Color::Black);
    } else {
        panic!("Expected check");
    }
}

#[test]
fn test_should_create_checkmate() {
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

    let third_row = [
        None,
        None,
        None,
        Some(ChessPiece::create_queen(Color::White)),
        None,
        None,
        None,
        None,
    ];

    let fourth_row = [
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        None,
        None,
        None,
    ];

    let black_first_row = [
        Some(ChessPiece::create_rook(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        Some(ChessPiece::create_bishop(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_king(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_rook(Color::Black)),
    ];

    let black_second_row = [
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_pawn(Color::Black)),
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_white_row,
        white_second_row,
        third_row,
        fourth_row,
        [None; 8],
        [None; 8],
        black_second_row,
        black_first_row,
    ];

    let mut board = Board::mock(pieces, Color::White, None);

    let from = Position::from_str("d3").unwrap();
    let to = Position::from_str("e4").unwrap();

    assert!(board.move_piece(from, to));
    if let Some(check) = board.get_winner() {
        assert_eq!(check, Color::White);
    } else {
        panic!("Expected checkmate");
    }
}
