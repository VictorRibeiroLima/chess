use actix::Message;

use crate::lobby::RoomId;

#[derive(Message)]
#[rtype(result = "Vec<RoomId>")]
pub struct AvailableRooms;

#[derive(Message)]
#[rtype(result = "bool")]
pub struct AvailableRoom(pub RoomId);
