use actix::prelude::Message;

use crate::lobby::{client::Client, ClientId, RoomId};

#[derive(Message)]
#[rtype(result = "()")]
pub struct StringMessage(pub String);

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
