use actix::Message;

use crate::{
    commands::{self},
    lobby::{client::Client, ClientId, RoomId},
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect(pub Client);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect(pub ClientId);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Command {
    pub client_id: ClientId,
    pub command: commands::Command,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomMessage {
    pub room_id: RoomId,
    pub message: RoomMessageType,
}

pub enum RoomMessageType {
    Full,
    Empty,
    Disconnect,
}
