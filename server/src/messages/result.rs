use actix::Message;
use engine::{
    piece::{position::Position, ChessPiece, Color, Type},
    result::OkMovement,
};
use serde::Serialize;

use crate::lobby::{
    room::{Room, TurnMove},
    ClientId, RoomId,
};

#[derive(Message, Serialize, Clone)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub enum ResultMessage {
    Error(ErrorMessage),
    Success(SuccessMessage),
    Timer(TimerMessage),
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
        turn_number: u32,
    ) -> Self {
        let result = MovementResult {
            movement_type: movement,
            promotion,
            check,
            turn_number,
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

    pub fn connect(client_id: ClientId, con_type: ConnectionType, room: &Room) -> Self {
        let room_id = room.id();
        let (pieces, moves) = match con_type {
            ConnectionType::EnemyClient => (None, None),
            ConnectionType::SelfClient => (Some(room.pieces()), Some(room.moves().clone())),
        };
        let check = room.check();
        let promotion = room.promotion();

        let enemy = room.enemy(client_id).unwrap(); //TODO: Handle this better
        let enemy_id = enemy.map(|c| c.id());
        let color = room.get_color(client_id).unwrap(); //TODO: Handle this better

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
                moves,
                check,
                promotion,
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

    pub fn timer(room_id: RoomId, client_id: Option<ClientId>, time: u32) -> Self {
        Self::Timer(TimerMessage {
            client_id,
            room_id,
            time,
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

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuccessMessage {
    room_id: RoomId,
    client_id: ClientId,
    result: SuccessResult,
}

#[derive(Serialize, Clone)]
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
    turn_number: u32,
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

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ConnectSuccess {
    room_id: RoomId,
    client_id: ClientId,
    enemy_id: Option<ClientId>,
    con_type: ConnectionType,
    color: Color,
    check: Option<Color>,
    promotion: Option<Color>,
    pieces: Option<[[Option<ChessPiece>; 8]; 8]>,
    moves: Option<Vec<TurnMove>>, //Option so that we don't pass the moves to the client when enemy connects
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionType {
    EnemyClient,
    SelfClient,
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct TimerMessage {
    client_id: Option<ClientId>, //Client id is optional because client can be disconnected
    room_id: RoomId,
    time: u32,
}
