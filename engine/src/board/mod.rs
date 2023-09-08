use std::fmt;

use crate::piece::{position::Position, ChessPiece, Color, Type};

pub struct Board {
    turn: Color,
    pieces: [[Option<ChessPiece>; 8]; 8],
    winner: Option<Color>,
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
        let first_white_row = [
            Some(ChessPiece::create_rook(Color::White)),
            Some(ChessPiece::create_knight(Color::White)),
            Some(ChessPiece::create_bishop(Color::White)),
            Some(ChessPiece::create_queen(Color::White)),
            Some(ChessPiece::create_king(Color::White)),
            Some(ChessPiece::create_bishop(Color::White)),
            Some(ChessPiece::create_knight(Color::White)),
            Some(ChessPiece::create_rook(Color::White)),
        ];

        let second_white_row = [
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
        ];

        let first_black_row = [
            Some(ChessPiece::create_rook(Color::Black)),
            Some(ChessPiece::create_knight(Color::Black)),
            Some(ChessPiece::create_bishop(Color::Black)),
            Some(ChessPiece::create_queen(Color::Black)),
            Some(ChessPiece::create_king(Color::Black)),
            Some(ChessPiece::create_bishop(Color::Black)),
            Some(ChessPiece::create_knight(Color::Black)),
            Some(ChessPiece::create_rook(Color::Black)),
        ];

        let second_black_row = [
            Some(ChessPiece::create_pawn(Color::Black)),
            Some(ChessPiece::create_pawn(Color::Black)),
            Some(ChessPiece::create_pawn(Color::Black)),
            Some(ChessPiece::create_pawn(Color::Black)),
            Some(ChessPiece::create_pawn(Color::Black)),
            Some(ChessPiece::create_pawn(Color::Black)),
            Some(ChessPiece::create_pawn(Color::Black)),
            Some(ChessPiece::create_pawn(Color::Black)),
        ];

        let pieces: [[Option<ChessPiece>; 8]; 8] = [
            first_white_row,
            second_white_row,
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            second_black_row,
            first_black_row,
        ];

        Board {
            turn: Color::White,
            pieces,
            winner: None,
        }
    }

    #[cfg(test)]
    pub fn mock(pieces: [[Option<ChessPiece>; 8]; 8], turn: Color) -> Board {
        Board {
            turn,
            pieces,
            winner: None,
        }
    }

    pub fn get_winner(&self) -> Option<Color> {
        self.winner
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
            Some(piece) => {
                let can_move = piece.can_move(from, to, self);
                if can_move {
                    self.make_movement(piece, from, to);
                }
                return can_move;
            }
            None => false,
        }
    }

    pub fn get_piece_at(&self, position: &Position) -> Option<&ChessPiece> {
        let piece = self.pieces[position.y as usize][position.x as usize].as_ref();
        piece
    }

    pub fn is_vertical_path_clean(&self, from: &Position, to: &Position) -> bool {
        let mut y = from.y;
        let x = from.x;

        loop {
            if y < to.y {
                y += 1;
            } else {
                y -= 1;
            }

            let position = Position { x, y };

            if position == *to {
                break;
            }

            if self.get_piece_at(&position).is_some() {
                return false;
            }
        }

        true
    }

    pub fn is_horizontal_path_clean(&self, from: &Position, to: &Position) -> bool {
        let mut x = from.x;
        let y = from.y;

        loop {
            if x < to.x {
                x += 1;
            } else {
                x -= 1;
            }

            let position = Position { x, y };
            if position == *to {
                break;
            }
            if self.get_piece_at(&position).is_some() {
                return false;
            }
        }

        true
    }

    pub fn is_diagonal_path_clean(&self, from: &Position, to: &Position) -> bool {
        let mut x = from.x;
        let mut y = from.y;

        loop {
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

            if position == *to {
                break;
            }

            if self.get_piece_at(&position).is_some() {
                return false;
            }
        }

        true
    }

    fn make_movement(&mut self, mut piece: ChessPiece, from: Position, to: Position) {
        piece.moved = true;
        let old_piece = self.pieces[to.y as usize][to.x as usize];
        self.pieces[from.y as usize][from.x as usize] = None;
        self.pieces[to.y as usize][to.x as usize] = Some(piece);

        if let Some(old_piece) = old_piece {
            if old_piece.get_type() == Type::King {
                self.winner = Some(self.turn);
            }
        }
        self.change_turn();
    }
}
