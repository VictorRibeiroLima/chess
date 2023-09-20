use serde::{Deserialize, Serialize};
use std::fmt::Display;

use colored::Colorize;

use crate::{
    board::Board,
    result::{Movement, MovementError, OkMovement},
};

use self::position::Position;

pub mod position;

#[cfg(test)]
mod test;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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

    pub fn create_king(color: Color) -> ChessPiece {
        ChessPiece::new(Type::King, color)
    }

    pub fn create_queen(color: Color) -> ChessPiece {
        ChessPiece::new(Type::Queen, color)
    }

    pub fn create_bishop(color: Color) -> ChessPiece {
        ChessPiece::new(Type::Bishop, color)
    }

    pub fn create_knight(color: Color) -> ChessPiece {
        ChessPiece::new(Type::Knight, color)
    }

    pub fn create_rook(color: Color) -> ChessPiece {
        ChessPiece::new(Type::Rook, color)
    }

    pub fn create_pawn(color: Color) -> ChessPiece {
        ChessPiece::new(Type::Pawn, color)
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_type(&self) -> Type {
        self.piece_type
    }

    pub fn can_move(&self, from: Position, to: Position, board: &Board) -> Movement {
        if from == to {
            return Err(MovementError::SamePosition);
        }
        let movement = match self.piece_type {
            Type::Pawn => self.can_move_pawn(&from, &to, board),
            Type::Bishop => self.can_move_bishop(&from, &to, board),
            Type::Rook => self.can_move_rock(&from, &to, board),
            Type::King => self.can_move_king(&from, &to, board),
            Type::Knight => self.can_move_knight(&from, &to, board),
            Type::Queen => self.can_move_queen(&from, &to, board),
        };

        if movement.is_err() {
            return movement;
        }

        let movement = movement.unwrap();

        let creates_check = board.creates_check(movement);

        if creates_check {
            return Err(MovementError::CreatesOwnCheck);
        }

        return Ok(movement);
    }

    //TODO: Too expensive, refactor
    /// Returns a list of legal moves for the piece at the given position
    pub fn legal_moves(&self, from: Position, board: &Board) -> Vec<Position> {
        let mut legal_moves = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                let to = Position { x, y };

                if self.can_move(from, to, board).is_ok() {
                    legal_moves.push(to);
                }
            }
        }

        legal_moves
    }

    fn can_move_pawn(&self, from: &Position, to: &Position, board: &Board) -> Movement {
        let x_diff = to.x - from.x;
        let y_diff = to.y - from.y;

        let piece_at_position = board.get_piece_at(to);
        let color = self.color;

        match (x_diff.abs(), y_diff, piece_at_position) {
            // No piece at the destination
            (0, 1, None) if color == Color::White => Ok(OkMovement::Valid((*from, *to))),
            (0, -1, None) if color == Color::Black => Ok(OkMovement::Valid((*from, *to))),

            // First move, allowing two squares
            (0, 2, None) => {
                let is_path_clear = board.is_vertical_path_clean(from, to);
                let is_color_white = color == Color::White;
                let is_first_move = !self.moved;
                let valid = is_path_clear && is_color_white && is_first_move;
                if valid {
                    Ok(OkMovement::InitialDoubleAdvance((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
            (0, -2, None) => {
                let is_path_clear = board.is_vertical_path_clean(from, to);
                let is_color_black = color == Color::Black;
                let is_first_move = !self.moved;
                let valid = is_path_clear && is_color_black && is_first_move;
                if valid {
                    Ok(OkMovement::InitialDoubleAdvance((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }

            // Capture diagonally (forward for white, backward for black)
            (1, 1, Some(other_piece)) if color == Color::White && color != other_piece.color => {
                Ok(OkMovement::Capture((*from, *to)))
            }
            (1, -1, Some(other_piece)) if color == Color::Black && color != other_piece.color => {
                Ok(OkMovement::Capture((*from, *to)))
            }

            //EnPassant
            (1, 1, None) => {
                let last_move = board.get_last_move();
                let last_move = match last_move {
                    Some(result) => result,
                    None => return Err(MovementError::InvalidMovement),
                };
                let last_move = match last_move {
                    Ok(result) => result,
                    Err(_) => return Err(MovementError::InvalidMovement),
                };
                let last_move = match last_move {
                    OkMovement::InitialDoubleAdvance((from, to)) => (from, to),
                    _ => return Err(MovementError::InvalidMovement),
                };
                let to_last_move = last_move.1;
                let is_color_white = color == Color::White;

                // If the moving piece's current Y-axis is the same as the last move's destination Y-axis
                // and the last move's destination X-axis is the same as the target X-axis,
                // then the last moved piece is going to be behind the moving piece
                let is_last_move_behind = to_last_move.y == from.y && to_last_move.x == to.x;

                let valid = is_color_white && is_last_move_behind;

                if valid {
                    Ok(OkMovement::EnPassant((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
            (1, -1, None) => {
                let last_move = board.get_last_move();
                let last_move = match last_move {
                    Some(result) => result,
                    None => return Err(MovementError::InvalidMovement),
                };
                let last_move = match last_move {
                    Ok(result) => result,
                    Err(_) => return Err(MovementError::InvalidMovement),
                };
                let last_move = match last_move {
                    OkMovement::InitialDoubleAdvance((from, to)) => (from, to),
                    _ => return Err(MovementError::InvalidMovement),
                };
                let to_last_move = last_move.1;
                let is_color_black = color == Color::Black;

                let is_last_move_behind = to_last_move.y == from.y && to_last_move.x == to.x;

                let valid = is_color_black && is_last_move_behind;

                if valid {
                    Ok(OkMovement::EnPassant((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }

            _ => Err(MovementError::InvalidMovement),
        }
    }

    fn can_move_bishop(&self, from: &Position, to: &Position, board: &Board) -> Movement {
        let x_diff = (to.x - from.x).abs();
        let y_diff = (to.y - from.y).abs();

        if x_diff != y_diff {
            return Err(MovementError::InvalidMovement);
        }

        let piece_at_position = board.get_piece_at(to);
        let color = self.color;

        let is_path_clear = board.is_diagonal_path_clean(from, to);

        if !is_path_clear {
            return Err(MovementError::InvalidMovement);
        }

        match piece_at_position {
            None => Ok(OkMovement::Valid((*from, *to))),
            Some(other_piece) => {
                let capture = color != other_piece.color;
                if capture {
                    Ok(OkMovement::Capture((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    fn can_move_rock(&self, from: &Position, to: &Position, board: &Board) -> Movement {
        let x_diff = (to.x - from.x).abs();
        let y_diff = (to.y - from.y).abs();

        if x_diff != 0 && y_diff != 0 {
            return Err(MovementError::InvalidMovement);
        }

        let is_path_clear = match (x_diff, y_diff) {
            (0, _) => board.is_vertical_path_clean(from, to),
            (_, 0) => board.is_horizontal_path_clean(from, to),
            _ => false,
        };

        if !is_path_clear {
            return Err(MovementError::InvalidMovement);
        }

        let piece_at_position = board.get_piece_at(to);

        match piece_at_position {
            None => Ok(OkMovement::Valid((*from, *to))),
            Some(other_piece) => {
                let capture = self.color != other_piece.color;
                if capture {
                    Ok(OkMovement::Capture((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    fn can_move_king(&self, from: &Position, to: &Position, board: &Board) -> Movement {
        let x_diff = to.x - from.x;
        let y_diff = to.y - from.y;

        //castling
        //see: https://en.wikipedia.org/wiki/Castling
        if y_diff == 0 && (x_diff == 2 || x_diff == -2) {
            return self.can_perform_castling(from, to, board);
        }

        let x_diff = x_diff.abs();
        let y_diff = y_diff.abs();

        if x_diff > 1 || y_diff > 1 {
            return Err(MovementError::InvalidMovement);
        }

        let piece_at_position = board.get_piece_at(to);

        match piece_at_position {
            None => Ok(OkMovement::Valid((*from, *to))),
            Some(other_piece) => {
                let capture = self.color != other_piece.color;
                if capture {
                    Ok(OkMovement::Capture((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    fn can_move_knight(&self, from: &Position, to: &Position, board: &Board) -> Movement {
        let x_diff = (to.x - from.x).abs();
        let y_diff = (to.y - from.y).abs();

        if x_diff == 0 || y_diff == 0 {
            return Err(MovementError::InvalidMovement);
        }

        if x_diff + y_diff != 3 {
            return Err(MovementError::InvalidMovement);
        }

        let piece_at_position = board.get_piece_at(to);

        match piece_at_position {
            None => Ok(OkMovement::Valid((*from, *to))),
            Some(other_piece) => {
                let capture = self.color != other_piece.color;
                if capture {
                    Ok(OkMovement::Capture((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    fn can_move_queen(&self, from: &Position, to: &Position, board: &Board) -> Movement {
        let x_diff = (to.x - from.x).abs();
        let y_diff = (to.y - from.y).abs();

        let piece_at_position = board.get_piece_at(to);
        let color = self.color;

        //The queen is moving at a L shape and not a straight line
        if x_diff != y_diff && x_diff != 0 && y_diff != 0 {
            return Err(MovementError::InvalidMovement);
        }

        let is_path_clear = match (x_diff, y_diff) {
            (0, _) => board.is_vertical_path_clean(from, to),
            (_, 0) => board.is_horizontal_path_clean(from, to),
            _ => board.is_diagonal_path_clean(from, to),
        };

        if !is_path_clear {
            return Err(MovementError::InvalidMovement);
        }

        match piece_at_position {
            None => Ok(OkMovement::Valid((*from, *to))),
            Some(other_piece) => {
                let capture = color != other_piece.color;
                if capture {
                    Ok(OkMovement::Capture((*from, *to)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    fn can_perform_castling(&self, from: &Position, to: &Position, board: &Board) -> Movement {
        let x_diff = to.x - from.x;

        let rock_from = match x_diff {
            2 => Position { x: 7, y: from.y },
            -2 => Position { x: 0, y: from.y },
            _ => return Err(MovementError::InvalidMovement),
        };
        let rock_to = match x_diff {
            2 => Position { x: 5, y: from.y },
            -2 => Position { x: 3, y: from.y },
            _ => return Err(MovementError::InvalidMovement),
        };

        //1. The king and the chosen rook are on the player's first rank.
        let rock = board.get_piece_at(&rock_from);
        let rock = match rock {
            Some(rock) => rock,
            None => return Err(MovementError::InvalidMovement),
        };

        //2. Neither the king nor the chosen rook has previously moved.
        if self.moved {
            return Err(MovementError::InvalidMovement);
        }
        if rock.moved {
            return Err(MovementError::InvalidMovement);
        }

        //3. There are no pieces between the king and the chosen rook.
        let is_path_clear = match x_diff {
            2 => board.is_horizontal_path_clean(from, &Position { x: 7, y: from.y }),
            -2 => board.is_horizontal_path_clean(from, &Position { x: 0, y: from.y }),
            _ => return Err(MovementError::InvalidMovement),
        };
        if !is_path_clear {
            return Err(MovementError::InvalidMovement);
        }

        //4. The king is not currently in check and does not pass through a square that is attacked by an enemy piece.
        let path_under_attack = match x_diff {
            2 => {
                board.is_position_been_attacked(Position { x: 4, y: from.y }, self.color)
                    || board.is_position_been_attacked(Position { x: 5, y: from.y }, self.color)
                    || board.is_position_been_attacked(Position { x: 6, y: from.y }, self.color)
            }
            -2 => {
                board.is_position_been_attacked(Position { x: 2, y: from.y }, self.color)
                    || board.is_position_been_attacked(Position { x: 3, y: from.y }, self.color)
                    || board.is_position_been_attacked(Position { x: 4, y: from.y }, self.color)
            }
            _ => return Err(MovementError::InvalidMovement),
        };
        if path_under_attack {
            return Err(MovementError::InvalidMovement);
        }

        return Ok(OkMovement::Castling((*from, *to), (rock_from, rock_to)));
    }
}
