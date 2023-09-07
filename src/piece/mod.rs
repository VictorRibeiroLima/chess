use std::fmt::Display;

use colored::Colorize;

use crate::board::Board;

use self::position::Position;

pub mod position;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self {
            Piece::Pawn => "♙",
            Piece::Knight => "♘",
            Piece::Bishop => "♗",
            Piece::Rook => "♖",
            Piece::Queen => "♕",
            Piece::King => "♔",
        };
        write!(f, "{}", piece)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Color::White => "White",
            Color::Black => "Black",
        };
        write!(f, "{}", color)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChessPiece {
    piece: Piece,
    color: Color,
    position: Position,
    moved: bool,
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_display = self.piece.to_string();
        match self.color {
            Color::White => write!(f, "{}", piece_display.white()),
            Color::Black => write!(f, "{}", piece_display.red()), // black pieces are red for better visibility
        }
    }
}

impl ChessPiece {
    pub fn new(piece: Piece, color: Color, position: Position) -> ChessPiece {
        ChessPiece {
            piece,
            color,
            position,
            moved: false,
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_piece(&self) -> &Piece {
        &self.piece
    }

    pub fn can_move(&self, position: Position, board: &Board) -> bool {
        if self.get_color() != board.get_turn() {
            return false;
        }
        let move_result = match self.piece {
            Piece::Pawn => move_pawn(self, &position, board),
            _ => false,
        };

        return move_result;
    }

    pub fn move_piece(&mut self, position: Position) {
        self.position = position;
        self.moved = true;
    }
}

fn move_pawn<'a>(piece: &ChessPiece, position: &Position, board: &'a Board) -> bool {
    let current_position = piece.get_position();
    let x_diff = position.x - current_position.x;
    let y_diff = position.y - current_position.y;

    let piece_at_position = board.get_piece_at(position);

    match piece_at_position {
        None => {
            //Whites can only move up the board,while blacks can only move down
            if piece.get_color() == &Color::White {
                if x_diff == 0 && y_diff == 1 {
                    return true;
                }
            } else {
                if x_diff == 0 && y_diff == -1 {
                    return true;
                }
            }

            //If is the first movement of the pawn we can move 2 squares
            if !piece.moved {
                if piece.get_color() == &Color::White {
                    if x_diff == 0 && y_diff == 2 {
                        return true;
                    }
                } else {
                    if x_diff == 0 && y_diff == -2 {
                        return true;
                    }
                }
            }
        }
        Some(other_piece) => {
            // Player can't place a piece on top of other piece of the same color
            if other_piece.color == piece.color {
                return false;
            }

            if piece.get_color() == &Color::White {
                if x_diff.abs() == 1 && y_diff == 1 {
                    return true;
                }
            } else {
                if x_diff.abs() == 1 && y_diff == -1 {
                    return true;
                }
            }
        }
    }

    return false;
}
