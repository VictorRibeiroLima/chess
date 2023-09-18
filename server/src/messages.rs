use actix::prelude::Message;

use crate::{
    commands::Command,
    lobby::{client::Client, ClientId, RoomId},
};

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
