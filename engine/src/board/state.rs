use crate::piece::{position::Position, Color};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    WaitingPromotion(Color, Position),
    InProgress,
    Winner(Color),
    Draw,
}
