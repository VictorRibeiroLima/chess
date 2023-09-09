use std::str::FromStr;

use crate::{
    board::Board,
    piece::{position::Position, ChessPiece, Color},
    result::{MovementError, OkMovement},
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

    let mut board = Board::mock(pieces, Color::White, None, None);

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

    let mut board = Board::mock(pieces, Color::White, None, None);

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

    let mut board = Board::mock(pieces, Color::White, None, None);

    let from = Position::from_str("d3").unwrap();
    let to = Position::from_str("e4").unwrap();

    assert!(board.move_piece(from, to));
    if let Some(check) = board.get_winner() {
        assert_eq!(check, Color::White);
    } else {
        panic!("Expected checkmate");
    }
}

#[test]
fn test_should_mark_double_advance() {
    let mut board = Board::new();
    let from = Position::from_str("e2").unwrap();
    let to = Position::from_str("e4").unwrap();

    assert!(board.move_piece(from, to));
    let last_move = board.get_last_move();
    let last_move = last_move.unwrap().unwrap();
    assert_eq!(last_move, OkMovement::InitialDoubleAdvance((from, to)));
}

#[test]
fn test_en_passant() {
    let mut board = Board::new();
    let from = Position::from_str("e2").unwrap();
    let to = Position::from_str("e4").unwrap();

    assert!(board.move_piece(from, to));
    let from = Position::from_str("d7").unwrap();
    let to = Position::from_str("d5").unwrap();

    assert!(board.move_piece(from, to));
    let from = Position::from_str("e4").unwrap();
    let to = Position::from_str("d5").unwrap();

    assert!(board.move_piece(from, to));

    let from = Position::from_str("e7").unwrap();
    let to = Position::from_str("e5").unwrap();

    assert!(board.move_piece(from, to));

    let from = Position::from_str("d5").unwrap();
    let to = Position::from_str("e6").unwrap();

    assert!(board.move_piece(from, to));

    let last_move = board.get_last_move();
    let last_move = last_move.unwrap().unwrap();
    assert_eq!(last_move, OkMovement::EnPassant((from, to)));
}

/*
  see: https://en.wikipedia.org/wiki/Rules_of_chess#En_passant
  When a pawn advances two squares on its initial move and ends the turn adjacent to an enemy pawn on the
  same rank, it may be captured en passant by the enemy pawn as if it had moved only one square.
  This capture is legal only on the move immediately following the pawn's advance.
*/
#[test]
fn test_missed_en_passant() {
    let mut board = Board::new();
    let from = Position::from_str("e2").unwrap();
    let to = Position::from_str("e4").unwrap();

    assert!(board.move_piece(from, to));
    let from = Position::from_str("d7").unwrap();
    let to = Position::from_str("d5").unwrap();

    assert!(board.move_piece(from, to));
    let from = Position::from_str("e4").unwrap();
    let to = Position::from_str("d5").unwrap();

    assert!(board.move_piece(from, to));

    let from = Position::from_str("e7").unwrap();
    let to = Position::from_str("e5").unwrap();

    assert!(board.move_piece(from, to));

    let from = Position::from_str("a2").unwrap();
    let to = Position::from_str("a4").unwrap();

    assert!(board.move_piece(from, to));

    let from = Position::from_str("a7").unwrap();
    let to = Position::from_str("a5").unwrap();

    assert!(board.move_piece(from, to));

    //En passant is not possible anymore

    let from = Position::from_str("d5").unwrap();
    let to = Position::from_str("e6").unwrap();

    assert!(!board.move_piece(from, to));

    let last_move = board.get_last_move();
    let last_move = last_move.unwrap().unwrap_err();
    assert_eq!(last_move, MovementError::InvalidMovement);
}
