use std::collections::HashMap;
use std::fmt;

use crate::piece::{position::Position, ChessPiece, Color, Type};

pub struct Board {
    turn: Color,
    pieces: HashMap<Position, ChessPiece>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "it is {}'s turn", self.turn)?;
        writeln!(f, "----------------")?;

        for y in (0..8).rev() {
            for x in 0..8 {
                let piece = self.get_piece_at(&Position { x, y });
                match piece {
                    Some(piece) => {
                        let piece = piece.to_string();
                        write!(f, "{} ", piece)?;
                    }
                    None => write!(f, ". ")?,
                }
                if x == 7 {
                    write!(f, "|{}", y + 1)?;
                }
            }
            writeln!(f)?;
            if y == 0 {
                writeln!(f, "----------------")?;
                writeln!(f, "a b c d e f g h")?;
            }
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = HashMap::new();

        // Populate the map with white pieces
        for x in 0..8 {
            pieces.insert(
                Position { x, y: 1 },
                ChessPiece::new(Type::Pawn, Color::White),
            );
        }

        // Populate the map with black pieces
        for x in 0..8 {
            pieces.insert(
                Position { x, y: 6 },
                ChessPiece::new(Type::Pawn, Color::Black),
            );
        }

        //Whites
        pieces.insert(
            Position { x: 0, y: 0 },
            ChessPiece::new(Type::Rook, Color::White),
        );
        pieces.insert(
            Position { x: 1, y: 0 },
            ChessPiece::new(Type::Knight, Color::White),
        );
        pieces.insert(
            Position { x: 2, y: 0 },
            ChessPiece::new(Type::Bishop, Color::White),
        );
        pieces.insert(
            Position { x: 3, y: 0 },
            ChessPiece::new(Type::Queen, Color::White),
        );
        pieces.insert(
            Position { x: 4, y: 0 },
            ChessPiece::new(Type::King, Color::White),
        );
        pieces.insert(
            Position { x: 5, y: 0 },
            ChessPiece::new(Type::Bishop, Color::White),
        );
        pieces.insert(
            Position { x: 6, y: 0 },
            ChessPiece::new(Type::Knight, Color::White),
        );
        pieces.insert(
            Position { x: 7, y: 0 },
            ChessPiece::new(Type::Rook, Color::White),
        );

        //Blacks
        pieces.insert(
            Position { x: 0, y: 7 },
            ChessPiece::new(Type::Rook, Color::Black),
        );
        pieces.insert(
            Position { x: 1, y: 7 },
            ChessPiece::new(Type::Knight, Color::Black),
        );
        pieces.insert(
            Position { x: 2, y: 7 },
            ChessPiece::new(Type::Bishop, Color::Black),
        );
        pieces.insert(
            Position { x: 3, y: 7 },
            ChessPiece::new(Type::Queen, Color::Black),
        );
        pieces.insert(
            Position { x: 4, y: 7 },
            ChessPiece::new(Type::King, Color::Black),
        );
        pieces.insert(
            Position { x: 5, y: 7 },
            ChessPiece::new(Type::Bishop, Color::Black),
        );
        pieces.insert(
            Position { x: 6, y: 7 },
            ChessPiece::new(Type::Knight, Color::Black),
        );
        pieces.insert(
            Position { x: 7, y: 7 },
            ChessPiece::new(Type::Rook, Color::Black),
        );

        Board {
            turn: Color::White,
            pieces,
        }
    }

    pub fn change_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_turn(&self) -> &Color {
        &self.turn
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> bool {
        let piece = self.get_piece_at(&from).cloned();
        if from == to {
            return false;
        }
        match piece {
            Some(mut piece) => {
                let can_move = piece.can_move(from, to, self);
                if can_move {
                    piece.moved = true;
                    self.pieces.remove(&from);
                    self.pieces.insert(to, piece);
                    self.change_turn();
                }
                return can_move;
            }
            None => false,
        }
    }

    pub fn get_piece_at(&self, position: &Position) -> Option<&ChessPiece> {
        self.pieces.get(position)
    }

    pub fn is_vertical_path_clean(&self, from: &Position, to: &Position) -> bool {
        let mut y = from.y;
        let x = from.x;

        while y != to.y {
            if y < to.y {
                y += 1;
            } else {
                y -= 1;
            }

            let position = Position { x, y };
            if self.get_piece_at(&position).is_some() {
                return false;
            }
        }

        true
    }

    pub fn is_horizontal_path_clean(&self, from: &Position, to: &Position) -> bool {
        let mut x = from.x;
        let y = from.y;

        while x != to.x {
            if x < to.x {
                x += 1;
            } else {
                x -= 1;
            }

            let position = Position { x, y };
            if self.get_piece_at(&position).is_some() {
                return false;
            }
        }

        true
    }

    pub fn is_diagonal_path_clean(&self, from: &Position, to: &Position) -> bool {
        let mut x = from.x;
        let mut y = from.y;

        while x != to.x && y != to.y {
            if x < to.x {
                x += 1;
            } else {
                x -= 1;
            }

            if y < to.y {
                y += 1;
            } else {
                y -= 1;
            }

            let position = Position { x, y };
            if self.get_piece_at(&position).is_some() {
                return false;
            }
        }

        true
    }
}
