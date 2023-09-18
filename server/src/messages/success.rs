use actix::Message;
use engine::{
    piece::{position::Position, Color, Type},
    result::OkMovement,
};
use serde::Serialize;

use crate::lobby::{ClientId, RoomId};

#[derive(Message, Serialize, Clone, Copy)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub struct SuccessMessage {
    pub room_id: RoomId,
    pub client_id: ClientId,
    pub result: SuccessResult,
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SuccessResult {
    Movement(OkMovement),
    Promotion((Position, Type)),
    Disconnect(DisconnectSuccess),
    Connect(ConnectSuccess),
}

#[derive(Serialize, Clone, Copy)]
pub struct DisconnectSuccess {
    pub room_id: RoomId,
    pub client_id: ClientId,
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ConnectSuccess {
    pub room_id: RoomId,
    pub client_id: ClientId,
    pub color: Color,
}
