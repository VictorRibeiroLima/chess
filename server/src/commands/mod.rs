use std::fmt::Display;

use engine::piece::{position::Position, Type};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    Move { from: Position, to: Position },
    Promote { piece: Type },
    Resign(bool),
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    InvalidCommand,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Error::InvalidCommand => "Invalid command",
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for Error {}
