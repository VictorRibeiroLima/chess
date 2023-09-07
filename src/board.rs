use std::collections::HashMap;
use std::fmt;

use crate::piece::{position::Position, ChessPiece, Color, Piece};

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
                ChessPiece::new(Piece::Pawn, Color::White, Position { x, y: 1 }),
            );
        }

        // Populate the map with black pieces
        for x in 0..8 {
            pieces.insert(
                Position { x, y: 6 },
                ChessPiece::new(Piece::Pawn, Color::Black, Position { x, y: 6 }),
            );
        }

        //Whites
        pieces.insert(
            Position { x: 0, y: 0 },
            ChessPiece::new(Piece::Rook, Color::White, Position { x: 0, y: 0 }),
        );
        pieces.insert(
            Position { x: 1, y: 0 },
            ChessPiece::new(Piece::Knight, Color::White, Position { x: 1, y: 0 }),
        );
        pieces.insert(
            Position { x: 2, y: 0 },
            ChessPiece::new(Piece::Bishop, Color::White, Position { x: 2, y: 0 }),
        );
        pieces.insert(
            Position { x: 3, y: 0 },
            ChessPiece::new(Piece::Queen, Color::White, Position { x: 3, y: 0 }),
        );
        pieces.insert(
            Position { x: 4, y: 0 },
            ChessPiece::new(Piece::King, Color::White, Position { x: 4, y: 0 }),
        );
        pieces.insert(
            Position { x: 5, y: 0 },
            ChessPiece::new(Piece::Bishop, Color::White, Position { x: 5, y: 0 }),
        );
        pieces.insert(
            Position { x: 6, y: 0 },
            ChessPiece::new(Piece::Knight, Color::White, Position { x: 6, y: 0 }),
        );
        pieces.insert(
            Position { x: 7, y: 0 },
            ChessPiece::new(Piece::Rook, Color::White, Position { x: 0, y: 0 }),
        );

        //Blacks
        pieces.insert(
            Position { x: 0, y: 7 },
            ChessPiece::new(Piece::Rook, Color::Black, Position { x: 0, y: 7 }),
        );
        pieces.insert(
            Position { x: 1, y: 7 },
            ChessPiece::new(Piece::Knight, Color::Black, Position { x: 1, y: 7 }),
        );
        pieces.insert(
            Position { x: 2, y: 7 },
            ChessPiece::new(Piece::Bishop, Color::Black, Position { x: 2, y: 7 }),
        );
        pieces.insert(
            Position { x: 3, y: 7 },
            ChessPiece::new(Piece::Queen, Color::Black, Position { x: 3, y: 7 }),
        );
        pieces.insert(
            Position { x: 4, y: 7 },
            ChessPiece::new(Piece::King, Color::Black, Position { x: 4, y: 7 }),
        );
        pieces.insert(
            Position { x: 5, y: 7 },
            ChessPiece::new(Piece::Bishop, Color::Black, Position { x: 5, y: 7 }),
        );
        pieces.insert(
            Position { x: 6, y: 7 },
            ChessPiece::new(Piece::Knight, Color::Black, Position { x: 6, y: 7 }),
        );
        pieces.insert(
            Position { x: 7, y: 7 },
            ChessPiece::new(Piece::Rook, Color::Black, Position { x: 0, y: 7 }),
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
        match piece {
            Some(mut piece) => {
                let moved = piece.can_move(to, self);
                piece.move_piece(to);
                if moved {
                    self.pieces.remove(&from);
                    self.pieces.insert(to, piece);
                    self.change_turn();
                }
                return moved;
            }
            None => false,
        }
    }

    pub fn get_piece_at(&self, position: &Position) -> Option<&ChessPiece> {
        self.pieces.get(position)
    }
}
