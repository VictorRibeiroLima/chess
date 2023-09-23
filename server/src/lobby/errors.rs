use std::{error::Error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug)]
pub enum RoomError {
    RoomNotFound,
    ClientNotFound,
    RoomFull,
    ClientNotInRoom,
    ClientAlreadyInRoom,
    ClientAlreadyInOtherRoom,
    NotYourTurn,
    NotEnoughPlayers,
    GameNotOver,
}

impl Display for RoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomError::RoomNotFound => write!(f, "Room not found"),
            RoomError::ClientNotFound => write!(f, "Client not found"),
            RoomError::RoomFull => write!(f, "Room is full"),
            RoomError::ClientNotInRoom => write!(f, "Client is not in room"),
            RoomError::ClientAlreadyInRoom => write!(f, "Client is already in room"),
            RoomError::ClientAlreadyInOtherRoom => write!(f, "Client is already in another room"),
            RoomError::NotYourTurn => write!(f, "It is not your turn"),
            RoomError::NotEnoughPlayers => write!(f, "Not enough players"),
            RoomError::GameNotOver => write!(f, "Game is not over"),
        }
    }
}

impl Error for RoomError {}
