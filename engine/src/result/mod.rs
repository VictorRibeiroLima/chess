use std::{error::Error, fmt::Display};

use crate::piece::position::Position;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MovementError {
    InvalidMovement,
    InvalidPiece,
    CheckNotResolved,
    CreatesOwnCheck,
    SamePosition,
}

impl Display for MovementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            MovementError::InvalidMovement => "Invalid movement",
            MovementError::InvalidPiece => "Invalid piece",
            MovementError::CheckNotResolved => "Check not resolved",
            MovementError::CreatesOwnCheck => "Creates own check",
            MovementError::SamePosition => "Same position",
        };
        write!(f, "{}", error_message)
    }
}

impl Error for MovementError {}

/// The Ok variant of the Movement
/// Valid((Position, Position)) - A valid movement (from, to)
/// Capture((Position, Position)) - A valid capture movement (from, to)
/// EnPassant((Position, Position)) - A valid en passant movement (from, to)
/// Promotion((Position, Position)) - A valid promotion movement (from, to)
/// Castling((Position, Position), (Position, Position)) - A valid castling movement (king, rock) (from, to)
/// InitialDoubleAdvance((Position, Position)) - A valid initial double advance of a pawn movement (from, to)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OkMovement {
    Valid((Position, Position)),
    Capture((Position, Position)),
    EnPassant((Position, Position)),
    Promotion((Position, Position)),
    Castling((Position, Position), (Position, Position)),
    InitialDoubleAdvance((Position, Position)),
}

pub type Movement = Result<OkMovement, MovementError>;
