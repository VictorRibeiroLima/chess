use serde::{Deserialize, Serialize};
use std::fmt::Display;

use colored::Colorize;

use crate::{
    bit_magic,
    board::{state::Color, Board},
    result::{MovementError, MovementType, OkMovement},
};

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChessPiece {
    #[serde(rename = "type")]
    piece_type: Type,
    color: Color,
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
        ChessPiece { piece_type, color }
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

    pub fn can_move(&self, from: u64, to: u64, board: &Board) -> Result<OkMovement, MovementError> {
        let movement = self.sudo_legal_move(from, to, board)?;

        let creates_check = board.creates_check(movement);

        if creates_check {
            return Err(MovementError::CreatesOwnCheck);
        }

        return Ok(movement);
    }

    //TODO: Too expensive, refactor
    /// Returns a list of legal moves for the piece at the given position
    pub fn legal_moves(&self, from: u64, board: &Board) -> Vec<u64> {
        let mut legal_moves = Vec::new();

        for x in 0..64 {
            let to = 1 << x;

            if self.can_move(from, to, board).is_ok() {
                legal_moves.push(to);
            }
        }

        legal_moves
    }

    pub fn attack_board(&self, from: u64, board: &Board) -> u64 {
        let mut attack_board = 0;

        for x in 0..8 {
            let to = 1 << x;

            if self.sudo_legal_move(from, to, board).is_ok() {
                attack_board |= to;
            }
        }

        attack_board
    }

    fn sudo_legal_move(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        if from == to {
            return Err(MovementError::SamePosition);
        }

        let movement = match self.piece_type {
            Type::Pawn => self.can_move_pawn(from, to, board),
            Type::Bishop => self.can_move_bishop(from, to, board),
            Type::Rook => self.can_move_rock(from, to, board),
            Type::King => self.can_move_king(from, to, board),
            Type::Knight => self.can_move_knight(from, to, board),
            Type::Queen => self.can_move_queen(from, to, board),
        };

        return movement;
    }

    fn can_move_pawn(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        let piece_at_position = board.get_piece_at(to);
        let color = self.color;
        let is_moving_forward = bit_magic::is_moving_vertically(from, to);

        if is_moving_forward {
            return self.forward_pawn_movement(from, to, board);
        } else {
            todo!()
        }

        /*
        match (is_at_initial_position, y_diff, piece_at_position) {
            // No piece at the destination
            (0, 1, None) if color == Color::White => Ok(self.valid(from, to)),
            (0, -1, None) if color == Color::Black => Ok(self.valid(from, to)),

            // First move, allowing two squares
            (true, 2, None) => {
                let is_path_clear = board.is_vertical_path_clean(from, to);
                let is_color_white = color == Color::White;
                //if the pawn is white, it should be at the second row
                let is_first_move = from.y == 1;
                let valid = is_path_clear && is_color_white && is_first_move;
                if valid {
                    Ok(self.initial_double_advance(from, to))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
            (true, -2, None) => {
                let is_path_clear = board.is_vertical_path_clean(from, to);
                let is_color_black = color == Color::Black;
                //if the pawn is black, it should be at the seventh row
                let is_first_move = from.y == 6;
                let valid = is_path_clear && is_color_black && is_first_move;
                if valid {
                    Ok(self.initial_double_advance(from, to))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }

            // Capture diagonally (forward for white, backward for black)
            (1, 1, Some(other_piece)) if color == Color::White && color != other_piece.color => {
                Ok(self.capture(from, to, other_piece))
            }
            (1, -1, Some(other_piece)) if color == Color::Black && color != other_piece.color => {
                Ok(self.capture(from, to, other_piece))
            }

            //EnPassant
            (1, 1, None) => {
                let en_passant = board.en_passant();
                let en_passant = match en_passant {
                    Some(result) => result,
                    None => return Err(MovementError::InvalidMovement),
                };

                let is_color_white = color == Color::White;

                // If the moving piece's current Y-axis is the same as the last move's destination Y-axis
                // and the last move's destination X-axis is the same as the target X-axis,
                // then the last moved piece is going to be behind the moving piece
                let valid_en_passant = en_passant.y == from.y && en_passant.x == to.x;

                let valid = is_color_white && valid_en_passant;

                if valid {
                    Ok(self.en_passant(from, to, ChessPiece::create_pawn(Color::Black)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
            (1, -1, None) => {
                let en_passant = board.en_passant();
                let en_passant = match en_passant {
                    Some(result) => result,
                    None => return Err(MovementError::InvalidMovement),
                };

                let is_color_black = color == Color::Black;

                let valid_en_passant = en_passant.y == from.y && en_passant.x == to.x;

                let valid = is_color_black && valid_en_passant;

                if valid {
                    Ok(self.en_passant(from, to, ChessPiece::create_pawn(Color::White)))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }

            _ => Err(MovementError::InvalidMovement),

        }
        */
    }

    fn can_move_bishop(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        let is_moving_diagonally = bit_magic::is_moving_diagonally(from, to);

        if !is_moving_diagonally {
            return Err(MovementError::InvalidMovement);
        }

        let piece_at_position = board.get_piece_at(to);
        let color = self.color;

        let is_path_clear = board.is_diagonal_path_clean(from, to);

        if !is_path_clear {
            return Err(MovementError::InvalidMovement);
        }

        match piece_at_position {
            None => Ok(self.valid(from, to)),
            Some(other_piece) => {
                let capture = color != other_piece.color;
                if capture {
                    Ok(self.capture(from, to, other_piece))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    fn can_move_rock(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        let is_moving_horizontally = bit_magic::is_moving_horizontally(from, to);
        let is_moving_vertically = bit_magic::is_moving_vertically(from, to);

        if !is_moving_horizontally && !is_moving_vertically {
            return Err(MovementError::InvalidMovement);
        }

        let is_path_clear = match (is_moving_vertically, is_moving_horizontally) {
            (true, _) => board.is_vertical_path_clean(from, to),
            (_, true) => board.is_horizontal_path_clean(from, to),
            _ => false,
        };

        if !is_path_clear {
            return Err(MovementError::InvalidMovement);
        }

        let piece_at_position = board.get_piece_at(to);

        match piece_at_position {
            None => Ok(self.valid(from, to)),
            Some(other_piece) => {
                let capture = self.color != other_piece.color;
                if capture {
                    Ok(self.capture(from, to, other_piece))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    fn can_move_king(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        let is_moving_horizontally = bit_magic::is_moving_horizontally(from, to);
        let is_moving_vertically = bit_magic::is_moving_vertically(from, to);
        let is_moving_diagonally = bit_magic::is_moving_diagonally(from, to);
        let bit_distance = bit_magic::bit_distance(from, to);

        if !is_moving_horizontally && !is_moving_vertically && !is_moving_diagonally {
            return Err(MovementError::InvalidMovement);
        }

        if is_moving_vertically {
            if bit_distance != 8 {
                return Err(MovementError::InvalidMovement);
            }
        } else if is_moving_horizontally {
            //castling
            //see: https://en.wikipedia.org/wiki/Castling
            if bit_distance == 2 {
                todo!("Castling")
            }
            if bit_distance != 1 {
                return Err(MovementError::InvalidMovement);
            }
        } else {
            if bit_distance != 7 && bit_distance != 9 {
                return Err(MovementError::InvalidMovement);
            }
        }

        let piece_at_position = board.get_piece_at(to);

        match piece_at_position {
            None => Ok(self.valid(from, to)),
            Some(other_piece) => {
                let capture = self.color != other_piece.color;
                if capture {
                    Ok(self.capture(from, to, other_piece))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    fn can_move_knight(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        todo!();
        /*
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
            None => Ok(self.valid(from, to)),
            Some(other_piece) => {
                let capture = self.color != other_piece.color;
                if capture {
                    Ok(self.capture(from, to, other_piece))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
        */
    }

    fn can_move_queen(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        let is_moving_horizontally = bit_magic::is_moving_horizontally(from, to);
        let is_moving_vertically = bit_magic::is_moving_vertically(from, to);
        let is_moving_diagonally = bit_magic::is_moving_diagonally(from, to);

        let piece_at_position = board.get_piece_at(to);
        let color = self.color;

        //The queen is moving at a L shape and not a straight line
        if !is_moving_horizontally && !is_moving_vertically && !is_moving_diagonally {
            return Err(MovementError::InvalidMovement);
        }

        let is_path_clear = match (is_moving_vertically, is_moving_horizontally) {
            (true, _) => board.is_vertical_path_clean(from, to),
            (_, true) => board.is_horizontal_path_clean(from, to),
            _ => board.is_diagonal_path_clean(from, to),
        };

        if !is_path_clear {
            return Err(MovementError::InvalidMovement);
        }

        match piece_at_position {
            None => Ok(self.valid(from, to)),
            Some(other_piece) => {
                let capture = color != other_piece.color;
                if capture {
                    Ok(self.capture(from, to, other_piece))
                } else {
                    Err(MovementError::InvalidMovement)
                }
            }
        }
    }

    /*
    fn can_perform_castling(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        let x_diff = to.x - from.x;

        let rock_from = match x_diff {
            2 => u64 { x: 7, y: from.y },
            -2 => u64 { x: 0, y: from.y },
            _ => return Err(MovementError::InvalidMovement),
        };
        let rock_to = match x_diff {
            2 => u64 { x: 5, y: from.y },
            -2 => u64 { x: 3, y: from.y },
            _ => return Err(MovementError::InvalidMovement),
        };

        //1. The king and the chosen rook are on the player's first rank.
        let rock = board.get_piece_at(rock_from);
        let rock = match rock {
            Some(rock) => rock,
            None => return Err(MovementError::InvalidMovement),
        };

        if rock.color != self.color {
            return Err(MovementError::InvalidMovement);
        }

        //2. Neither the king nor the chosen rook has previously moved.
        if !board.can_castle(self.color) {
            return Err(MovementError::InvalidMovement);
        }

        //3. There are no pieces between the king and the chosen rook.
        let is_path_clear = match x_diff {
            2 => board.is_horizontal_path_clean(from, to),
            -2 => board.is_horizontal_path_clean(from, to),
            _ => return Err(MovementError::InvalidMovement),
        };
        if !is_path_clear {
            return Err(MovementError::InvalidMovement);
        }

        //4. The king is not currently in check and does not pass through a square that is attacked by an enemy piece.
        let path_under_attack = match x_diff {
            2 => {
                board.is_position_been_attacked(to, self.color)
                    || board.is_position_been_attacked(to, self.color)
                    || board.is_position_been_attacked(to, self.color)
            }
            -2 => {
                board.is_position_been_attacked(to, self.color)
                    || board.is_position_been_attacked(to, self.color)
                    || board.is_position_been_attacked(to, self.color)
            }
            _ => return Err(MovementError::InvalidMovement),
        };

        println!("path_under_attack: {}", path_under_attack);
        if path_under_attack {
            return Err(MovementError::InvalidMovement);
        }

        return Ok(self.castling(from, to, rock_from, rock_to));
    }
    */

    fn forward_pawn_movement(
        &self,
        from: u64,
        to: u64,
        board: &Board,
    ) -> Result<OkMovement, MovementError> {
        let color = self.color;
        let piece_at_position = board.get_piece_at(to);
        match piece_at_position {
            Some(_) => return Err(MovementError::InvalidMovement),
            None => match color {
                Color::White => {
                    let valid = (to == from << 8)
                        || (to == from << 16
                            && bit_magic::is_pawn_at_initial_position(from, color));

                    let is_path_clear = board.is_vertical_path_clean(from, to);
                    if valid && is_path_clear {
                        return Ok(self.valid(from, to));
                    } else {
                        return Err(MovementError::InvalidMovement);
                    }
                }
                Color::Black => {
                    let valid = (to == from >> 8)
                        || (to == from >> 16
                            && bit_magic::is_pawn_at_initial_position(from, color));

                    let is_path_clear = board.is_vertical_path_clean(from, to);
                    if valid && is_path_clear {
                        return Ok(self.valid(from, to));
                    } else {
                        return Err(MovementError::InvalidMovement);
                    }
                }
            },
        };
    }

    fn valid(&self, from: u64, to: u64) -> OkMovement {
        OkMovement {
            from,
            to,
            movement_type: MovementType::Valid,
            mover: *self,
        }
    }

    fn capture(&self, from: u64, to: u64, captured_piece: ChessPiece) -> OkMovement {
        OkMovement {
            from,
            to,
            movement_type: MovementType::Capture(captured_piece),
            mover: *self,
        }
    }

    fn en_passant(&self, from: u64, to: u64, captured_piece: ChessPiece) -> OkMovement {
        OkMovement {
            from,
            to,
            movement_type: MovementType::EnPassant(captured_piece),
            mover: *self,
        }
    }

    fn initial_double_advance(&self, from: u64, to: u64) -> OkMovement {
        OkMovement {
            from,
            to,
            movement_type: MovementType::InitialDoubleAdvance,
            mover: *self,
        }
    }

    fn castling(&self, from: u64, to: u64, rock_from: u64, rock_to: u64) -> OkMovement {
        OkMovement {
            from,
            to,
            movement_type: MovementType::Castling(rock_from, rock_to),
            mover: *self,
        }
    }
}
