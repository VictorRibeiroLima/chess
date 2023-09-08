use std::fmt;

use crate::piece::{position::Position, ChessPiece, Color, Type};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Board {
    turn: Color,
    pieces: [[Option<ChessPiece>; 8]; 8],
    winner: Option<Color>,
    check: Option<Color>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "it is {}'s turn", self.turn)?;
        if let Some(check) = self.check {
            writeln!(f, "{} is in check", check)?;
        }
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
        Board {
            turn: Color::White,
            pieces: Board::initial_pieces_setup(),
            winner: None,
            check: None,
        }
    }

    pub fn reset(&mut self) {
        self.turn = Color::White;
        self.pieces = Board::initial_pieces_setup();
        self.winner = None;
        self.check = None;
    }

    #[cfg(test)]
    pub fn mock(pieces: [[Option<ChessPiece>; 8]; 8], turn: Color, check: Option<Color>) -> Board {
        Board {
            turn,
            pieces,
            winner: None,
            check,
        }
    }

    pub fn get_winner(&self) -> Option<Color> {
        self.winner
    }

    pub fn change_turn(&mut self) {
        self.turn = self.next_turn();
    }

    pub fn next_turn(&self) -> Color {
        match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn get_check(&self) -> Option<Color> {
        self.check
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> bool {
        let piece = self.get_piece_at(&from).cloned();
        if from == to {
            return false;
        }
        match piece {
            Some(piece) => {
                if piece.get_color() != self.turn {
                    return false;
                }
                let can_move = piece.can_move(from, to, self);
                if can_move {
                    let removed_piece = self.make_movement(piece, from, to);
                    let game_over = Board::game_over(removed_piece);

                    if game_over {
                        self.winner = Some(self.turn);
                        return true;
                    }

                    let enemy_color = self.next_turn();

                    if self.is_king_in_check(enemy_color) {
                        self.check = Some(enemy_color);
                        let is_checkmate = self.is_checkmate(enemy_color);
                        if is_checkmate {
                            self.winner = Some(self.turn);
                            return true;
                        }
                    }

                    self.change_turn();
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

    pub fn removes_check(&self, from: Position, to: Position) -> bool {
        let piece = self.get_piece_at(&from).cloned();
        if from == to {
            return false;
        }
        match piece {
            Some(piece) => {
                let mut board = self.clone();
                let removed_piece = board.make_movement(piece, from, to);
                let game_over = Board::game_over(removed_piece);

                if game_over {
                    return true;
                }
                let is_king_in_check = board.is_king_in_check(piece.get_color());
                return !is_king_in_check;
            }
            None => false,
        }
    }

    fn is_king_in_check(&self, king_color: Color) -> bool {
        let king_position = self.find_king_position(king_color).unwrap();
        let mut is_check = false;

        for y in 0..8 {
            for x in 0..8 {
                let position = Position { x, y };
                let piece = self.get_piece_at(&position);
                if let Some(piece) = piece {
                    let piece_color = piece.get_color();
                    if piece_color != king_color {
                        let can_move = piece.can_move(position, king_position, self);
                        if can_move {
                            is_check = true;
                        }
                    }
                }
            }
        }

        is_check
    }

    pub fn is_checkmate(&self, player_color: Color) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.pieces[y][x] {
                    if piece.get_color() == player_color {
                        let from: Position = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        let legal_moves = piece.legal_moves(from, self);
                        if !legal_moves.is_empty() {
                            return false;
                        }
                    }
                }
            }
        }

        true // No legal moves can remove the check, it's checkmate
    }

    fn find_king_position(&self, color: Color) -> Option<Position> {
        for y in 0..8 {
            for x in 0..8 {
                let position = Position { x, y };
                let piece = self.get_piece_at(&position);
                if let Some(piece) = piece {
                    let piece_type = piece.get_type();
                    let piece_color = piece.get_color();
                    if piece_type == Type::King && piece_color == color {
                        return Some(position);
                    }
                }
            }
        }
        None
    }

    ///Make a movement on the board, and returns the captured piece if there is one
    fn make_movement(
        &mut self,
        mut piece: ChessPiece,
        from: Position,
        to: Position,
    ) -> Option<ChessPiece> {
        piece.moved = true;
        let old_piece = self.pieces[to.y as usize][to.x as usize];
        self.pieces[from.y as usize][from.x as usize] = None;
        self.pieces[to.y as usize][to.x as usize] = Some(piece);

        return old_piece;
    }

    ///Checks if the removed piece is a king
    fn game_over(removed_piece: Option<ChessPiece>) -> bool {
        if let Some(old_piece) = removed_piece {
            if old_piece.get_type() == Type::King {
                return true;
            }
        }

        return false;
    }

    fn initial_pieces_setup() -> [[Option<ChessPiece>; 8]; 8] {
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

        return pieces;
    }
}