use engine::piece::{position::Position, Type};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    Move { from: Position, to: Position },
    Promote { piece: Type },
    Resign,
}

#[derive(Debug, serde::Serialize)]
pub enum Error {
    InvalidCommand,
}
