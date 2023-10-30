use std::fmt;

use crate::{
    piece::{position::Position, ChessPiece, Color, Type},
    result::{Movement, MovementError, OkMovement, PromotionError},
};

use self::state::{GameState, Turn};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, Clone)]
pub struct Board {
    turn: Color,
    pieces: [[Option<ChessPiece>; 8]; 8],
    state: GameState,
    check: Option<Color>,
    turns: Vec<Turn>,
    turn_number: u32,
    white_king_position: Position,
    black_king_position: Position,
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
            state: GameState::InProgress,
            check: None,
            turns: Vec::new(),
            turn_number: 0,
            white_king_position: Position { x: 4, y: 0 },
            black_king_position: Position { x: 4, y: 7 },
        }
    }

    #[cfg(test)]
    pub fn mock(
        pieces: [[Option<ChessPiece>; 8]; 8],
        turn: Color,
        check: Option<Color>,
        last_move: Option<OkMovement>,
    ) -> Board {
        let mut white_king_position = Position { x: 4, y: 0 };
        let mut black_king_position = Position { x: 4, y: 7 };

        for y in 0..8 {
            for x in 0..8 {
                let position = Position { x, y };
                let piece = pieces[y as usize][x as usize];
                if let Some(piece) = piece {
                    let piece_type = piece.get_type();
                    let piece_color = piece.get_color();
                    if piece_type == Type::King {
                        if piece_color == Color::White {
                            white_king_position = position;
                        } else {
                            black_king_position = position;
                        }
                    }
                }
            }
        }

        Board {
            turn,
            pieces,
            state: GameState::InProgress,
            check,
            turns: Vec::new(),
            turn_number: 0,
            white_king_position,
            black_king_position,
        }
    }

    pub fn reset(&mut self) {
        self.turn = Color::White;
        self.pieces = Board::initial_pieces_setup();
        self.state = GameState::InProgress;
        self.check = None;
        self.turns = Vec::new();
        self.turn_number = 0;
        self.white_king_position = Position { x: 4, y: 0 };
        self.black_king_position = Position { x: 4, y: 7 };
    }

    pub fn undo(&mut self) {
        if self.turns.is_empty() {
            return;
        }
        if self.turns.len() == 1 {
            self.reset();
            return;
        }
        let turn = self.turns.pop().unwrap(); // Safe to unwrap, we checked if it's empty
        match turn.movement {
            Movement::Move(movement) => {
                match movement {
                    OkMovement::Valid((from, to)) => {
                        let mut piece = self.pieces[to.y as usize][to.x as usize].unwrap();
                        piece.moves = piece.moves - 1;
                        self.pieces[to.y as usize][to.x as usize] = None;
                        self.pieces[from.y as usize][from.x as usize] = Some(piece);
                    }
                    OkMovement::Capture((from, to), captured_piece) => {
                        let mut piece = self.pieces[to.y as usize][to.x as usize].unwrap();
                        piece.moves = piece.moves - 1;
                        self.pieces[to.y as usize][to.x as usize] = Some(captured_piece);
                        self.pieces[from.y as usize][from.x as usize] = Some(piece);
                    }
                    OkMovement::EnPassant((from, to), captured_piece) => {
                        let mut piece = self.pieces[to.y as usize][to.x as usize].unwrap();
                        piece.moves = piece.moves - 1;
                        self.pieces[to.y as usize][to.x as usize] = None;
                        self.pieces[from.y as usize][from.x as usize] = Some(piece);
                        let enemy_pawn_position = Position { x: to.x, y: from.y };
                        self.pieces[enemy_pawn_position.y as usize]
                            [enemy_pawn_position.x as usize] = Some(captured_piece);
                    }
                    OkMovement::Castling(king, rock) => {
                        let king_from = king.0;
                        let king_to = king.1;
                        let rock_from = rock.0;
                        let rock_to = rock.1;

                        let mut rock = self.pieces[rock_to.y as usize][rock_to.x as usize].unwrap();
                        rock.moves = 0;
                        self.pieces[rock_from.y as usize][rock_from.x as usize] = Some(rock);
                        self.pieces[rock_to.y as usize][rock_to.x as usize] = None;

                        let mut king = self.pieces[king_to.y as usize][king_to.x as usize].unwrap();
                        king.moves = 0;
                        self.pieces[king_from.y as usize][king_from.x as usize] = Some(king);
                        self.pieces[king_to.y as usize][king_to.x as usize] = None;
                    }
                    OkMovement::InitialDoubleAdvance((from, to)) => {
                        let mut piece = self.pieces[to.y as usize][to.x as usize].unwrap();
                        piece.moves = 0;
                        self.pieces[to.y as usize][to.x as usize] = None;
                        self.pieces[from.y as usize][from.x as usize] = Some(piece);
                    }
                };
            }
            Movement::Promotion(on, _) => {
                let piece = self.pieces[on.y as usize][on.x as usize].unwrap();
                let color = piece.get_color();
                let mut pawn = ChessPiece::create_pawn(color);
                pawn.moves = piece.moves;

                self.pieces[on.y as usize][on.x as usize] = Some(pawn);
            }
        };
        let turn = self.turns.last().unwrap(); // Safe to unwrap, we checked if the length is > 1
        self.turn = turn.color;
        self.state = turn.state;
        self.check = turn.check;
        self.white_king_position = turn.white_king_position;
        self.black_king_position = turn.black_king_position;
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

    pub fn get_last_move(&self) -> Option<Movement> {
        self.turns.last().map(|turn| turn.movement)
    }

    pub fn get_turn_number(&self) -> u32 {
        self.turn_number
    }

    pub fn get_promotion_color(&self) -> Option<Color> {
        match self.state {
            GameState::WaitingPromotion(color, _) => Some(color),
            _ => None,
        }
    }

    pub fn get_pieces(&self) -> &[[Option<ChessPiece>; 8]; 8] {
        &self.pieces
    }

    /// Resign the game, and returns the winner
    pub fn resign(&mut self) -> Color {
        let winner = self.next_turn();
        self.state = GameState::Winner(winner);
        winner
    }

    //TODO: This is a very expensive operation.
    pub fn legal_moves(&self) -> Vec<(Position, Position)> {
        let mut moves = Vec::new();
        match self.state {
            GameState::WaitingPromotion(_, _) => return moves,
            GameState::Winner(_) => return moves,
            GameState::Draw => return moves,
            GameState::InProgress => {}
        }
        for y in 0..8 {
            for x in 0..8 {
                let position = Position { x, y };
                let piece = self.get_piece_at(&position);
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
        }
        moves
    }

    pub fn promote(&mut self, mut piece: ChessPiece) -> Result<(Position, Type), PromotionError> {
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

        let old_piece = self.pieces[position.y as usize][position.x as usize].unwrap();

        piece.moves = old_piece.moves;

        self.pieces[position.y as usize][position.x as usize] = Some(piece);
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
        self.turns
            .push(Turn::promotion(position, piece_type, &self));
        return Ok((position, piece_type));
    }

    pub fn move_piece(
        &mut self,
        from: Position,
        to: Position,
    ) -> Result<OkMovement, MovementError> {
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

        let piece = self.get_piece_at(&from).cloned();
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
                    self.turns.push(Turn::movement(movement, &self));
                }

                return movement;
            }
            None => Err(MovementError::InvalidPiece),
        }
    }

    pub fn get_piece_at(&self, position: &Position) -> Option<&ChessPiece> {
        // Invalid position
        if position.x < 0 || position.x > 7 || position.y < 0 || position.y > 7 {
            return None;
        }
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

    pub fn creates_check(&self, movement: OkMovement) -> bool {
        let mut board = self.clone();
        let moved_piece = board.make_movement(movement);

        match board.state {
            GameState::Winner(_) => return false,
            GameState::Draw => return false,
            _ => {}
        }

        let is_king_in_check = board.is_king_in_check(moved_piece.get_color());
        return is_king_in_check;
    }

    //TODO: keep track of attacked positions of each player to avoid this expensive operation
    pub fn is_position_been_attacked(&self, target: Position, color: Color) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let position = Position { x, y };
                let piece = self.get_piece_at(&position);
                if let Some(piece) = piece {
                    let piece_color = piece.get_color();
                    if piece_color != color {
                        let movement = piece.can_move(position, target, self);
                        let can_move = movement.is_ok();
                        if can_move {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn is_checkmate(&self, player_color: Color) -> bool {
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

    fn is_king_in_check(&self, king_color: Color) -> bool {
        let king_position = self.find_king_position(king_color);

        let is_check = self.is_position_been_attacked(king_position, king_color);

        is_check
    }

    fn find_king_position(&self, color: Color) -> Position {
        match color {
            Color::White => self.white_king_position,
            Color::Black => self.black_king_position,
        }
    }

    ///Make a movement on the board, and returns the moved piece
    fn make_movement(&mut self, movement: OkMovement) -> ChessPiece {
        let (from, to) = match movement {
            OkMovement::EnPassant((from, to), _) => {
                let enemy_pawn_position = Position { x: to.x, y: from.y };
                self.pieces[enemy_pawn_position.y as usize][enemy_pawn_position.x as usize] = None;
                (from, to)
            }
            OkMovement::Castling(king, rock) => {
                let king_from = king.0;
                let king_to = king.1;
                let rock_from = rock.0;
                let rock_to = rock.1;

                let mut rock = self.pieces[rock_from.y as usize][rock_from.x as usize].unwrap();
                rock.moves = 1;
                self.pieces[rock_from.y as usize][rock_from.x as usize] = None;
                self.pieces[rock_to.y as usize][rock_to.x as usize] = Some(rock);
                (king_from, king_to)
            }
            OkMovement::Capture((from, to), captured_piece) => {
                if captured_piece.get_type() == Type::King {
                    if captured_piece.get_color() == Color::White {
                        self.state = GameState::Winner(Color::Black);
                    } else {
                        self.state = GameState::Winner(Color::White);
                    }
                }
                (from, to)
            }
            OkMovement::InitialDoubleAdvance((from, to)) => (from, to),
            OkMovement::Valid((from, to)) => (from, to),
        };

        let mut piece = self.pieces[from.y as usize][from.x as usize].unwrap();
        piece.moves = piece.moves + 1;
        self.pieces[from.y as usize][from.x as usize] = None;
        self.pieces[to.y as usize][to.x as usize] = Some(piece);

        if piece.get_type() == Type::King {
            if piece.get_color() == Color::White {
                self.white_king_position = to;
            } else {
                self.black_king_position = to;
            }
        };

        return piece;
    }

    fn check_promotion(&self, piece: ChessPiece, target: Position) -> bool {
        let piece_type = piece.get_type();
        let piece_color = piece.get_color();
        if piece_type == Type::Pawn {
            if (piece_color == Color::White && target.y == 7)
                || (piece_color == Color::Black && target.y == 0)
            {
                return true;
            }
        }
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

pub mod state {
    use crate::{
        piece::{position::Position, Color, Type},
        result::{Movement, OkMovement},
    };

    use super::Board;

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum GameState {
        WaitingPromotion(Color, Position),
        InProgress,
        Winner(Color),
        Draw,
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Turn {
        pub color: Color,
        pub number: u32,
        pub movement: Movement,
        pub state: GameState,
        pub check: Option<Color>,
        pub white_king_position: Position,
        pub black_king_position: Position,
    }

    impl Turn {
        pub fn promotion(position: Position, piece: Type, board: &Board) -> Self {
            let color = board.turn;
            let turn = Turn {
                color,
                number: board.turn_number,
                movement: Movement::Promotion(position, piece),
                state: GameState::InProgress,
                check: None,
                white_king_position: board.white_king_position,
                black_king_position: board.black_king_position,
            };
            return turn;
        }

        pub fn movement(movement: OkMovement, board: &Board) -> Self {
            let color = board.turn;
            let turn = Turn {
                color,
                number: board.turn_number,
                movement: Movement::Move(movement),
                state: GameState::InProgress,
                check: None,
                white_king_position: board.white_king_position,
                black_king_position: board.black_king_position,
            };
            return turn;
        }
    }
}
