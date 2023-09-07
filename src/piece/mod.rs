use std::fmt::Display;

use colored::Colorize;

use crate::board::Board;

use self::position::Position;

pub mod position;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self {
            Type::Pawn => "♙",
            Type::Knight => "♘",
            Type::Bishop => "♗",
            Type::Rook => "♖",
            Type::Queen => "♕",
            Type::King => "♔",
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
    piece_type: Type,
    color: Color,
    pub moved: bool,
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_display = self.piece_type.to_string();
        match self.color {
            Color::White => write!(f, "{}", piece_display.white()),
            Color::Black => write!(f, "{}", piece_display.red()), // black pieces are red for better visibility
        }
    }
}

impl ChessPiece {
    pub fn new(piece_type: Type, color: Color) -> ChessPiece {
        ChessPiece {
            piece_type,
            color,
            moved: false,
        }
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn can_move(&self, from: Position, to: Position, board: &Board) -> bool {
        if self.get_color() != board.get_turn() {
            return false;
        }
        let move_result = match self.piece_type {
            Type::Pawn => can_move_pawn(self, &from, &to, board),
            _ => false,
        };

        return move_result;
    }
}

fn can_move_pawn(piece: &ChessPiece, from: &Position, to: &Position, board: &Board) -> bool {
    let x_diff = to.x - from.x;
    let y_diff = to.y - from.y;

    let piece_at_position = board.get_piece_at(to);

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
