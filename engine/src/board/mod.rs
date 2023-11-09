use std::fmt;

use crate::{
    bit_board::BitBoard,
    bit_magic,
    piece::{position::Position, ChessPiece, Type},
    result::{MovementError, MovementType, OkMovement, PromotionError},
};

pub mod state;

use self::state::{Color, GameState};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Board {
    turn: Color,
    state: GameState,
    check: Option<Color>,
    bit_board: BitBoard,
    en_passant: Option<u64>,
    white_castling_possible: bool,
    black_castling_possible: bool,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "it is {}'s turn", self.turn)?;
        if let Some(check) = self.check {
            writeln!(f, "{} is in check", check)?;
        }
        writeln!(f, "----------------")?;

        for i in (0..64).rev() {
            let position = 1 << i;
            let piece = self.get_piece_at(position);
            match piece {
                Some(piece) => {
                    let piece = piece.to_string();
                    write!(f, "{} ", piece)?;
                }
                None => write!(f, ". ")?,
            }
            if i % 8 == 0 {
                write!(f, "|{}", i / 8 + 1)?;
                writeln!(f)?;
            }

            if i == 0 {
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
            state: GameState::InProgress,
            check: None,
            bit_board: BitBoard::new(),
            en_passant: None,
            white_castling_possible: true,
            black_castling_possible: true,
        }
    }

    #[cfg(test)]
    pub fn mock(
        arr: [[Option<ChessPiece>; 8]; 8],
        turn: Color,
        check: Option<Color>,
        _: Option<OkMovement>,
    ) -> Board {
        Board {
            turn,
            state: GameState::InProgress,
            check,
            bit_board: BitBoard::from_array(arr),
            en_passant: None,
            white_castling_possible: true,
            black_castling_possible: true,
        }
    }

    pub fn reset(&mut self) {
        self.turn = Color::White;
        self.state = GameState::InProgress;
        self.check = None;
        self.bit_board = BitBoard::new();
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn get_check(&self) -> Option<Color> {
        self.check
    }

    pub fn can_castle(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_castling_possible,
            Color::Black => self.black_castling_possible,
        }
    }

    pub fn get_promotion_color(&self) -> Option<Color> {
        match self.state {
            GameState::WaitingPromotion(color, _) => Some(color),
            _ => None,
        }
    }

    pub fn get_pieces(&self) -> [[Option<ChessPiece>; 8]; 8] {
        self.bit_board.to_array()
    }

    /// Resign the game, and returns the winner
    pub fn resign(&mut self) -> Color {
        let winner = self.next_turn();
        self.state = GameState::Winner(winner);
        winner
    }

    //TODO: This is a very expensive operation.
    pub fn legal_moves(&self) -> Vec<(u64, u64)> {
        let mut moves = Vec::new();
        match self.state {
            GameState::WaitingPromotion(_, _) => return moves,
            GameState::Winner(_) => return moves,
            GameState::Draw => return moves,
            GameState::InProgress => {}
        }
        for i in 0..64 {
            let position = 1 << i;
            let piece = self.get_piece_at(position);
            if let Some(piece) = piece {
                let piece_color = piece.get_color();
                if piece_color == self.turn {
                    let legal_moves = piece.legal_moves(position, self);
                    for legal_move in legal_moves {
                        moves.push((position, legal_move));
                    }
                }
            }
        }
        moves
    }

    pub fn promote(&mut self, piece: ChessPiece) -> Result<(Position, Type), PromotionError> {
        let piece_type = piece.get_type();
        // can't promote to a pawn or king
        if piece_type == Type::Pawn || piece_type == Type::King {
            return Err(PromotionError::InvalidPromotion(piece_type));
        }

        let position = match self.state {
            GameState::WaitingPromotion(color, position) => {
                if color != self.turn {
                    return Err(PromotionError::NoPromotion);
                }
                position
            }
            GameState::Winner(_) => return Err(PromotionError::NoPromotion),
            GameState::Draw => return Err(PromotionError::NoPromotion),
            GameState::InProgress => return Err(PromotionError::NoPromotion),
        };

        self.bit_board.remove_piece_at_position(position);
        self.bit_board.add_piece_at_position(piece, position);

        self.state = GameState::InProgress;

        self.change_turn();
        let enemy_color = self.next_turn();
        if self.is_king_in_check(enemy_color) {
            self.check = Some(enemy_color);
            let is_checkmate = self.is_checkmate(enemy_color);
            if is_checkmate {
                self.state = GameState::Winner(self.turn);
            }
        } else {
            self.check = None;
        }
        let position = Position::from_bit_board(position);
        return Ok((position, piece_type));
    }

    pub fn move_piece(
        &mut self,
        from: Position,
        to: Position,
    ) -> Result<OkMovement, MovementError> {
        let from = from.to_bit_board();
        let to = to.to_bit_board();
        let err = match self.state {
            GameState::WaitingPromotion(_, _) => Some(MovementError::PromotionNotSpecified),
            GameState::Winner(_) => Some(MovementError::GameIsOver),
            GameState::Draw => Some(MovementError::GameIsOver),
            GameState::InProgress => {
                if from == to {
                    Some(MovementError::SamePosition)
                } else {
                    None
                }
            }
        };

        if let Some(error) = err {
            //self.last_move = Some(Err(error));
            return Err(error);
        }

        let piece = self.get_piece_at(from);
        match piece {
            Some(piece) => {
                if piece.get_color() != self.turn {
                    return Err(MovementError::InvalidPiece);
                }

                let movement = piece.can_move(from, to, self);
                let can_move = movement.is_ok();
                if can_move {
                    let movement = movement.unwrap();
                    self.make_movement(movement);
                    let enemy_color = self.next_turn();

                    match self.state {
                        GameState::Winner(_) => return Ok(movement),
                        GameState::Draw => return Ok(movement),
                        _ => {}
                    };

                    if self.is_king_in_check(enemy_color) {
                        self.check = Some(enemy_color);
                        let is_checkmate = self.is_checkmate(enemy_color);
                        if is_checkmate {
                            self.state = GameState::Winner(self.turn);
                        }
                    } else {
                        self.check = None;
                    }

                    let promotion = self.check_promotion(piece, to);
                    if promotion {
                        self.state = GameState::WaitingPromotion(self.turn, to);
                    } else {
                        self.change_turn();
                    }
                }

                return movement;
            }
            None => Err(MovementError::InvalidPiece),
        }
    }

    pub fn get_piece_at(&self, position: u64) -> Option<ChessPiece> {
        self.bit_board.piece_at_bit_board(position)
    }

    pub fn is_vertical_path_clean(&self, from: u64, to: u64) -> bool {
        let mov_bb = bit_magic::vertical_move_bb(from, to);
        let board = self.bit_board.full_board();

        mov_bb & board == 0
    }

    pub fn is_horizontal_path_clean(&self, from: u64, to: u64) -> bool {
        let mov_bb = bit_magic::horizontal_move_bb(from, to);
        let board = self.bit_board.full_board();

        mov_bb & board == 0
    }

    pub fn is_diagonal_path_clean(&self, from: u64, to: u64) -> bool {
        let mov_bb = bit_magic::diagonal_move_bb(from, to);
        let board = self.bit_board.full_board();

        mov_bb & board == 0
    }

    pub fn creates_check(&self, movement: OkMovement) -> bool {
        let mut board = *self;
        let moved_piece = board.make_movement(movement);

        match board.state {
            GameState::Winner(_) => return false,
            GameState::Draw => return false,
            _ => {}
        }

        let is_king_in_check = board.is_king_in_check(moved_piece.get_color());
        return is_king_in_check;
    }

    pub fn en_passant(&self) -> Option<u64> {
        self.en_passant
    }

    pub fn is_position_been_attacked(&self, target: u64, defender: Color) -> bool {
        let attacker = defender.other();
        let attack_board = self.color_attack_board(attacker);
        attack_board & target != 0
    }

    fn is_checkmate(&self, player_color: Color) -> bool {
        let color_board = match player_color {
            Color::White => self.bit_board.white_pieces(),
            Color::Black => self.bit_board.black_pieces(),
        };

        for i in 0..64 {
            let position = 1 << i;
            if color_board & position != 0 {
                let piece = self.get_piece_at(position).unwrap();
                let legal_moves = piece.legal_moves(position, self);

                if legal_moves.len() > 0 {
                    return false;
                }
            }
        }

        true // No legal moves can remove the check, it's checkmate
    }

    fn is_king_in_check(&self, king_color: Color) -> bool {
        let king_position = self.find_king_position(king_color);

        let is_check = self.is_position_been_attacked(king_position, king_color);

        is_check
    }

    fn find_king_position(&self, color: Color) -> u64 {
        match color {
            Color::White => self.bit_board.white_king,
            Color::Black => self.bit_board.black_king,
        }
    }

    ///Make a movement on the board, and returns the moved piece
    fn make_movement(&mut self, movement: OkMovement) -> ChessPiece {
        let from = movement.from;
        let to = movement.to;
        let piece = movement.mover;
        match (piece.get_color(), piece.get_type()) {
            (Color::White, Type::King) => self.white_castling_possible = false,
            (Color::White, Type::Rook) => self.white_castling_possible = false,

            (Color::Black, Type::King) => self.black_castling_possible = false,
            (Color::Black, Type::Rook) => self.black_castling_possible = false,

            _ => {}
        }
        self.bit_board.remove_piece_at_position(to);
        self.bit_board.move_piece(piece, from, to);
        match movement.movement_type {
            MovementType::EnPassant(_) => {
                let enemy_pawn_position = 0; //TODO: get enemy pawn position
                self.bit_board.remove_piece_at_position(enemy_pawn_position);
            }
            MovementType::Castling(rock_from, rock_to) => {
                let rock = self.bit_board.piece_at_bit_board(rock_from).unwrap();
                self.bit_board.move_piece(rock, rock_from, rock_to);
            }
            MovementType::Capture(captured_piece) => {
                if captured_piece.get_type() == Type::King {
                    if captured_piece.get_color() == Color::White {
                        self.state = GameState::Winner(Color::Black);
                    } else {
                        self.state = GameState::Winner(Color::White);
                    }
                }
            }
            MovementType::InitialDoubleAdvance => self.en_passant = Some(to),
            _ => self.en_passant = None,
        };

        return piece;
    }

    fn check_promotion(&self, piece: ChessPiece, target: u64) -> bool {
        let piece_type = piece.get_type();
        let piece_color = piece.get_color();
        /* if piece_type == Type::Pawn {
            if (piece_color == Color::White && target.y == 7)
                || (piece_color == Color::Black && target.y == 0)
            {
                return true;
            }
        }*/
        //TODO: check promotion
        false
    }

    fn change_turn(&mut self) {
        self.turn = self.next_turn();
    }

    fn next_turn(&self) -> Color {
        match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn color_attack_board(&self, color: Color) -> u64 {
        let mut attack_board = 0;
        let color_board = match color {
            Color::White => self.bit_board.white_pieces(),
            Color::Black => self.bit_board.black_pieces(),
        };
        for i in 0..64 {
            let position = 1 << i;
            if color_board & position != 0 {
                let piece = self.get_piece_at(position).unwrap();
                let piece_attack_board = piece.attack_board(position, self);
                attack_board |= piece_attack_board;
            }
        }
        attack_board
    }
}
