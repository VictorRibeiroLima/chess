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

#[test]
fn test_should_checkmate(){
    let first_white_row = [
        None, 
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_rook(Color::White)),
        Some(ChessPiece::create_king(Color::White)),
        None
    ];

    let second_white_row = [
        None, 
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_pawn(Color::White)),
        Some(ChessPiece::create_queen(Color::Black))
    ];

    let fifth_row = [
        None, 
        None,
        None,
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_rook(Color::Black))
    ];

    let last_row = [
        None, 
        None,
        None,
        None,
        Some(ChessPiece::create_king(Color::Black)),
        None,
        None,
        None,
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_white_row,
        second_white_row,
        [None; 8],
        [None; 8],
        fifth_row,
        [None; 8],
        [None; 8],
        last_row
    ];

    let mut board = Board::mock(pieces, Color::Black, None, None);

    let from = Position::from_str("h2").unwrap();
    let to = Position::from_str("h1").unwrap();

    assert!(board.move_piece(from, to));

    let last_move = board.get_last_move();
    let last_move = last_move.unwrap().unwrap();

    assert_eq!(last_move, OkMovement::Valid((from, to)));
    assert_eq!(board.get_winner(), Some(Color::Black));
}

#[test]
fn test_fools_mate(){
    let mut board = Board::new();

    let from = Position::from_str("f2").unwrap();
    let to = Position::from_str("f3").unwrap();

    assert!(board.move_piece(from, to));

    let from = Position::from_str("e7").unwrap();
    let to = Position::from_str("e5").unwrap();

    assert!(board.move_piece(from, to));

    let from = Position::from_str("g2").unwrap();
    let to = Position::from_str("g4").unwrap();

    assert!(board.move_piece(from, to));

    let from = Position::from_str("d8").unwrap();
    let to = Position::from_str("h4").unwrap();

    assert!(board.move_piece(from, to));

    assert_eq!(board.get_winner(), Some(Color::Black));
}

#[test]
fn test_invalid_check_move() {
    let eight_row = [
        None,
        None,
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_king(Color::Black)),
        None,
    ];

    let first_row = [
        None,
        None,
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_king(Color::White)),
        Some(ChessPiece::create_rook(Color::White)),
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_row,
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        eight_row,
    ];

    let mut board = Board::mock(pieces, Color::Black, None, None);

    let from = Position::from_str("g8").unwrap();
    let to = Position::from_str("h8").unwrap();

    assert!(!board.move_piece(from, to));

    let last_move = board.get_last_move();
    let last_move = last_move.unwrap().unwrap_err();

    assert_eq!(last_move, MovementError::CreatesOwnCheck);
}

#[test]
fn test_d_byrne_vs_fischer_checkmate(){
    let eight_row = [
        None,
        Some(ChessPiece::create_queen(Color::White)),
        None,
        None,
        None,
        None,
        None,
        None,
    ];

    let seventh_row = [
        None,
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        Some(ChessPiece::create_king(Color::Black)),
        None
    ];

    let sixth_row = [
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        None
    ];

    let fifth_row = [
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
        None,
        None,
        Some(ChessPiece::create_knight(Color::White)),
        None,
        None,
        Some(ChessPiece::create_pawn(Color::Black)),
    ];

    let fourth_row = [
        None,
        Some(ChessPiece::create_bishop(Color::Black)),
        None,
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::White)),
    ];

    let third_row = [
        None,
        Some(ChessPiece::create_bishop(Color::Black)),
        Some(ChessPiece::create_knight(Color::Black)),
        None,
        None,
        None,
        None,
        None
    ];

    let second_row = [
        Some(ChessPiece::create_rook(Color::Black)),
        None,
        None,
        None,
        None,
        None,
        Some(ChessPiece::create_pawn(Color::White)),
        None
    ];

    let first_row = [
        None,
        None,
        Some(ChessPiece::create_king(Color::White)),
        None,
        None,
        None,
        None,
        None
    ];

    let pieces: [[Option<ChessPiece>; 8]; 8] = [
        first_row,
        second_row,
        third_row,
        fourth_row,
        fifth_row,
        sixth_row,
        seventh_row,
        eight_row,
    ];

    let mut board = Board::mock(pieces, Color::Black, None, None);

    let from = Position::from_str("a2").unwrap();
    let to = Position::from_str("c2").unwrap();

    assert!(board.move_piece(from, to));
    assert_eq!(board.get_winner(), Some(Color::Black));
}