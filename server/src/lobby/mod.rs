use std::collections::{HashMap, HashSet};

use actix::{Actor, Addr, AsyncContext, Handler};

use uuid::Uuid;

pub mod client;
mod errors;
pub mod room;

pub type ClientId = Uuid;
pub type RoomId = Uuid;

use crate::messages::{
    inner::{AvailableRoom, AvailableRooms},
    CommandMessage, ConnectMessage, DisconnectMessage,
};

use self::room::{
    message::{RoomMessage, RoomMessageType},
    Room,
};

#[derive(Default, Clone)]
pub struct Lobby {
    available_rooms: HashSet<RoomId>,
    rooms: HashMap<RoomId, Addr<Room>>,
}

impl Lobby {
    pub fn available_room(&self, room_id: RoomId) -> bool {
        self.available_rooms.contains(&room_id)
    }

    pub fn available_rooms(&self) -> Vec<RoomId> {
        self.available_rooms.iter().cloned().collect()
    }
}

impl Actor for Lobby {
    type Context = actix::Context<Self>;
}

impl Handler<ConnectMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ConnectMessage, ctx: &mut Self::Context) -> Self::Result {
        let client = msg.client;
        let room_id = msg.room_id;
        let room = match self.rooms.get_mut(&room_id) {
            Some(room) => room,
            None => {
                let addr = ctx.address();
                let room = Room::new(room_id, addr.recipient());
                self.available_rooms.insert(room_id);
                let room = room.start();
                self.rooms.insert(room_id, room);
                self.rooms.get_mut(&room_id).unwrap()
            }
        };

        let room_msg = room::message::Connect(client);

        room.do_send(room_msg);
    }
}

impl Handler<RoomMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: RoomMessage, _: &mut Self::Context) -> Self::Result {
        let room_id = msg.room_id;

        let room_msg = msg.message;

        match room_msg {
            RoomMessageType::Full => {
                self.available_rooms.remove(&room_id);
            }
            RoomMessageType::Empty => {
                println!("Room {} is empty removing!", room_id);
                self.rooms.remove(&room_id);
                self.available_rooms.remove(&room_id);
                println!("Rooms: {}", self.rooms.len())
            }
            RoomMessageType::Disconnect => {
                self.available_rooms.insert(room_id);
            }
        }
    }
}

impl Handler<DisconnectMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: DisconnectMessage, _: &mut Self::Context) -> Self::Result {
        let room_id = msg.room_id;
        let client_id = msg.client_id;
        let room = match self.rooms.get_mut(&room_id) {
            Some(room) => room,
            None => {
                return;
            }
        };

        let room_msg = room::message::Disconnect(client_id);

        room.do_send(room_msg)
    }
}

impl Handler<CommandMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: CommandMessage, _: &mut Self::Context) -> Self::Result {
        let room_id = msg.room_id;
        let client_id = msg.client_id;
        let command = msg.command;
        let room = match self.rooms.get_mut(&room_id) {
            Some(room) => room,
            None => {
                return;
            }
        };

        let room_msg = room::message::Command { client_id, command };

        room.do_send(room_msg)
    }
}

impl Handler<AvailableRooms> for Lobby {
    type Result = Vec<RoomId>;

    fn handle(&mut self, _: AvailableRooms, _: &mut Self::Context) -> Self::Result {
        self.available_rooms()
    }
}

impl Handler<AvailableRoom> for Lobby {
    type Result = bool;

    fn handle(&mut self, msg: AvailableRoom, _: &mut Self::Context) -> Self::Result {
        return self.available_room(msg.0);
    }
}
