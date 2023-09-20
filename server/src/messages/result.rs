use actix::Message;
use engine::{
    piece::{position::Position, Color, Type},
    result::OkMovement,
};
use serde::Serialize;

use crate::lobby::{ClientId, RoomId};

#[derive(Message, Serialize, Clone)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub enum ResultMessage {
    Error(ErrorMessage),
    Success(SuccessMessage),
}

impl ResultMessage {
    pub fn error(room_id: RoomId, client_id: ClientId, error: String) -> Self {
        Self::Error(ErrorMessage {
            room_id,
            client_id,
            error,
        })
    }

    pub fn movement(room_id: RoomId, client_id: ClientId, movement: OkMovement) -> Self {
        Self::Success(SuccessMessage {
            room_id,
            client_id,
            result: SuccessResult::Movement(movement),
        })
    }

    pub fn promotion(room_id: RoomId, client_id: ClientId, promotion: (Position, Type)) -> Self {
        Self::Success(SuccessMessage {
            room_id,
            client_id,
            result: SuccessResult::Promotion(promotion),
        })
    }

    pub fn disconnect(room_id: RoomId, client_id: ClientId) -> Self {
        Self::Success(SuccessMessage {
            room_id,
            client_id,
            result: SuccessResult::Disconnect(DisconnectSuccess { room_id, client_id }),
        })
    }

    pub fn connect(room_id: RoomId, client_id: ClientId, color: Color) -> Self {
        Self::Success(SuccessMessage {
            room_id,
            client_id,
            result: SuccessResult::Connect(ConnectSuccess {
                room_id,
                client_id,
                color,
            }),
        })
    }

    pub fn winner(room_id: RoomId, client_id: ClientId, color: Color) -> Self {
        Self::Success(SuccessMessage {
            room_id,
            client_id,
            result: SuccessResult::Winner(color),
        })
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    room_id: RoomId,
    client_id: ClientId,
    error: String,
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct SuccessMessage {
    room_id: RoomId,
    client_id: ClientId,
    result: SuccessResult,
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
enum SuccessResult {
    Movement(OkMovement),
    Promotion((Position, Type)),
    Disconnect(DisconnectSuccess),
    Connect(ConnectSuccess),
    Winner(Color),
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct DisconnectSuccess {
    room_id: RoomId,
    client_id: ClientId,
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct ConnectSuccess {
    room_id: RoomId,
    client_id: ClientId,
    color: Color,
}
