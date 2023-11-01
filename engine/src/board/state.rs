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
