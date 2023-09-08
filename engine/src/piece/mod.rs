use std::fmt::Display;

use colored::Colorize;

use crate::board::Board;

use self::position::Position;

pub mod position;

#[cfg(test)]
mod test;

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

    pub fn can_move(&self, from: Position, to: Position, board: &Board) -> bool {
        let legal_movement = match self.piece_type {
            Type::Pawn => can_move_pawn(self, &from, &to, board),
            Type::Bishop => can_move_bishop(self, &from, &to, board),
            Type::Rook => can_move_rock(self, &from, &to, board),
            Type::King => can_move_king(self, &from, &to, board),
            Type::Knight => can_move_knight(self, &from, &to, board),
            Type::Queen => can_move_queen(self, &from, &to, board),
        };

        if !legal_movement {
            return false;
        }

        if let Some(check_color) = board.get_check() {
            if check_color == self.color {
                return board.removes_check(from, to);
            }
        }

        return !board.creates_check(from, to);
    }

    /// Returns a list of legal moves for the piece at the given position
    pub fn legal_moves(&self, from: Position, board: &Board) -> Vec<Position> {
        let mut legal_moves = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                let to = Position { x, y };

                if self.can_move(from, to, board) {
                    legal_moves.push(to);
                }
            }
        }

        legal_moves
    }
}

fn can_move_pawn(piece: &ChessPiece, from: &Position, to: &Position, board: &Board) -> bool {
    let x_diff = to.x - from.x;
    let y_diff = to.y - from.y;

    let piece_at_position = board.get_piece_at(to);
    let color = piece.color;

    match (x_diff.abs(), y_diff, piece_at_position) {
        // No piece at the destination
        (0, 1, None) if color == Color::White => true,
        (0, -1, None) if color == Color::Black => true,

        // First move, allowing two squares
        (0, 2, None) => {
            let is_path_clear = board.is_vertical_path_clean(from, to);
            let is_color_white = color == Color::White;
            let is_first_move = !piece.moved;
            is_path_clear && is_color_white && is_first_move
        }
        (0, -2, None) => {
            let is_path_clear = board.is_vertical_path_clean(from, to);
            let is_color_black = color == Color::Black;
            let is_first_move = !piece.moved;
            is_path_clear && is_color_black && is_first_move
        }

        // Capture diagonally (forward for white, backward for black)
        (1, 1, Some(other_piece)) if color == Color::White && color != other_piece.color => true,
        (1, -1, Some(other_piece)) if color == Color::Black && color != other_piece.color => true,

        _ => false,
    }
}

fn can_move_bishop(piece: &ChessPiece, from: &Position, to: &Position, board: &Board) -> bool {
    let x_diff = (to.x - from.x).abs();
    let y_diff = (to.y - from.y).abs();

    if x_diff != y_diff {
        return false;
    }

    let piece_at_position = board.get_piece_at(to);
    let color = piece.color;

    let is_path_clear = board.is_diagonal_path_clean(from, to);

    if !is_path_clear {
        return false;
    }

    match piece_at_position {
        None => true,
        Some(other_piece) => color != other_piece.color,
    }
}

fn can_move_rock(piece: &ChessPiece, from: &Position, to: &Position, board: &Board) -> bool {
    let x_diff = (to.x - from.x).abs();
    let y_diff = (to.y - from.y).abs();

    if x_diff != 0 && y_diff != 0 {
        return false;
    }

    let is_path_clear = match (x_diff, y_diff) {
        (0, _) => board.is_vertical_path_clean(from, to),
        (_, 0) => board.is_horizontal_path_clean(from, to),
        _ => false,
    };

    if !is_path_clear {
        return false;
    }

    let piece_at_position = board.get_piece_at(to);

    match piece_at_position {
        None => true,
        Some(other_piece) => piece.color != other_piece.color,
    }
}

pub fn can_move_king(piece: &ChessPiece, from: &Position, to: &Position, board: &Board) -> bool {
    let x_diff = (to.x - from.x).abs();
    let y_diff = (to.y - from.y).abs();

    if x_diff > 1 || y_diff > 1 {
        return false;
    }

    let piece_at_position = board.get_piece_at(to);

    match piece_at_position {
        None => true,
        Some(other_piece) => piece.color != other_piece.color,
    }
}

pub fn can_move_knight(piece: &ChessPiece, from: &Position, to: &Position, board: &Board) -> bool {
    let x_diff = (to.x - from.x).abs();
    let y_diff = (to.y - from.y).abs();

    if x_diff == 0 || y_diff == 0 {
        return false;
    }

    if x_diff + y_diff != 3 {
        return false;
    }

    let piece_at_position = board.get_piece_at(to);

    match piece_at_position {
        None => true,
        Some(other_piece) => piece.color != other_piece.color,
    }
}

pub fn can_move_queen(piece: &ChessPiece, from: &Position, to: &Position, board: &Board) -> bool {
    let x_diff = (to.x - from.x).abs();
    let y_diff = (to.y - from.y).abs();

    let piece_at_position = board.get_piece_at(to);
    let color = piece.color;

    //The queen is moving at a L shape and not a straight line
    if x_diff != y_diff && x_diff != 0 && y_diff != 0 {
        return false;
    }

    let is_path_clear = match (x_diff, y_diff) {
        (0, _) => board.is_vertical_path_clean(from, to),
        (_, 0) => board.is_horizontal_path_clean(from, to),
        _ => board.is_diagonal_path_clean(from, to),
    };

    if !is_path_clear {
        return false;
    }

    match piece_at_position {
        None => true,
        Some(other_piece) => color != other_piece.color,
    }
}
