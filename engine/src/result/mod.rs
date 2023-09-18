use serde::Serialize;
use std::{error::Error, fmt::Display};

use crate::piece::{position::Position, Type};

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
        };
        write!(f, "{}", error_message)
    }
}

impl Error for MovementError {}

/// The Ok variant of the Movement
/// Valid((Position, Position)) - A valid movement (from, to)
/// Capture((Position, Position)) - A valid capture movement (from, to)
/// EnPassant((Position, Position)) - A valid en passant movement (from, to)
/// Castling((Position, Position), (Position, Position)) - A valid castling movement (king, rock) (from, to)
/// InitialDoubleAdvance((Position, Position)) - A valid initial double advance of a pawn movement (from, to)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum OkMovement {
    Valid((Position, Position)),
    Capture((Position, Position)),
    EnPassant((Position, Position)),
    Castling((Position, Position), (Position, Position)),
    InitialDoubleAdvance((Position, Position)),
}

pub type Movement = Result<OkMovement, MovementError>;
