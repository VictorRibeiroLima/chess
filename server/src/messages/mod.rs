use actix::prelude::Message;

use serde::{Deserialize, Serialize};

pub mod success;

use crate::{
    commands::Command,
    lobby::{client::Client, ClientId, RoomId},
};

#[derive(Message)]
#[rtype(result = "Vec<RoomId>")]
pub struct AvailableRooms;

#[derive(Message)]
#[rtype(result = "bool")]
pub struct AvailableRoom(pub RoomId);

#[derive(Message)]
#[rtype(result = "()")]
pub struct StringMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct CommandMessage {
    pub room_id: RoomId,
    pub client_id: ClientId,
    pub command: Command,
}

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectMessage {
    pub room_id: RoomId,
    pub client: Client,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectMessage {
    pub room_id: RoomId,
    pub client_id: ClientId,
}

#[derive(Message, Deserialize, Serialize)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub room_id: RoomId,
    pub client_id: ClientId,
    pub error: String,
}
