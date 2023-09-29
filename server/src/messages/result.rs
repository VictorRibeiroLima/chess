use actix::Message;
use engine::{
    piece::{position::Position, ChessPiece, Color, Type},
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

    pub fn movement(
        room_id: RoomId,
        client_id: ClientId,
        movement: OkMovement,
        promotion: Option<Color>,
        check: Option<Color>,
    ) -> Self {
        let result = MovementResult {
            movement_type: movement,
            promotion,
            check,
        };
        Self::Success(SuccessMessage {
            room_id,
            client_id,
            result: SuccessResult::Movement(result),
        })
    }

    pub fn promotion(
        room_id: RoomId,
        client_id: ClientId,
        promotion: (Position, Type),
        check: Option<Color>,
    ) -> Self {
        let promotion = PromotionResult {
            position: promotion.0,
            piece: promotion.1,
            check,
        };
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

    pub fn connect(
        room_id: RoomId,
        client_id: ClientId,
        enemy_id: Option<ClientId>,
        con_type: ConnectionType,
        pieces: [[Option<ChessPiece>; 8]; 8],
        color: Color,
    ) -> Self {
        Self::Success(SuccessMessage {
            room_id,
            client_id,
            result: SuccessResult::Connect(ConnectSuccess {
                room_id,
                client_id,
                enemy_id,
                con_type,
                color,
                pieces,
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

    pub fn reset(room_id: RoomId, client_id: ClientId) -> Self {
        Self::Success(SuccessMessage {
            room_id,
            client_id,
            result: SuccessResult::Reset(true),
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
    Movement(MovementResult),
    Promotion(PromotionResult),
    Disconnect(DisconnectSuccess),
    Connect(ConnectSuccess),
    Winner(Color),
    Reset(bool),
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct MovementResult {
    #[serde(flatten)]
    movement_type: OkMovement,
    promotion: Option<Color>,
    check: Option<Color>,
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct PromotionResult {
    position: Position,
    piece: Type,
    check: Option<Color>,
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
    enemy_id: Option<ClientId>,
    con_type: ConnectionType,
    color: Color,
    pieces: [[Option<ChessPiece>; 8]; 8],
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionType {
    EnemyClient,
    SelfClient,
}
