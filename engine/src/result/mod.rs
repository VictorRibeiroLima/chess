use serde::Serialize;
use std::{error::Error, fmt::Display};

use crate::piece::{ChessPiece, Type};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PromotionError {
    InvalidPromotion(Type),
    NoPromotion,
}

impl Display for PromotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            PromotionError::InvalidPromotion(piece) => format!("Invalid promotion to: {}", piece),
            PromotionError::NoPromotion => "No promotion".to_string(),
        };
        write!(f, "{}", error_message)
    }
}

impl Error for PromotionError {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MovementError {
    InvalidMovement,
    InvalidPiece,
    CheckNotResolved,
    CreatesOwnCheck,
    SamePosition,
    PromotionNotSpecified,
    GameIsOver,
}

impl Display for MovementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            MovementError::InvalidMovement => "Invalid movement",
            MovementError::InvalidPiece => "Invalid piece",
            MovementError::CheckNotResolved => "Check not resolved",
            MovementError::CreatesOwnCheck => "Creates own check",
            MovementError::SamePosition => "Same position",
            MovementError::PromotionNotSpecified => "Promotion not specified",
            MovementError::GameIsOver => "Game is over",
        };
        write!(f, "{}", error_message)
    }
}

impl Error for MovementError {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub struct OkMovement {
    pub mover: ChessPiece,
    pub movement_type: MovementType,
    pub from: u64,
    pub to: u64,
}

/// The Ok variant of the Movement
/// Valid - A valid movement
/// CaptureChessPiece) - A valid capture movement (Captured piece)
/// EnPassant() - A valid en passant movement (Captured piece)
/// Castling((Position, Position)) - A valid castling movement (rook from, rook to)
/// InitialDoubleAdvance - A valid initial double advance of a pawn movement
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MovementType {
    Valid,
    Capture(ChessPiece),
    EnPassant(ChessPiece),
    Castling(u64, u64),
    InitialDoubleAdvance,
}
