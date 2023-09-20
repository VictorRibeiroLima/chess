use std::{collections::HashMap, sync::Arc};

use actix::{Actor, Handler};
use uuid::Uuid;

pub mod client;
mod errors;
pub mod room;

pub type ClientId = Uuid;
pub type RoomId = Uuid;

use crate::{
    commands::Command,
    messages::{
        result::ResultMessage, AvailableRoom, AvailableRooms, CommandMessage, ConnectMessage,
        DisconnectMessage,
    },
};

use self::{client::Client, room::Room};

#[derive(Default, Clone)]
pub struct Lobby {
    sessions: HashMap<ClientId, Arc<Client>>,
    rooms: HashMap<RoomId, Room>,
}

impl Lobby {
    pub fn send_room_result(&self, room_id: RoomId, result: ResultMessage) {
        let room = match self.rooms.get(&room_id) {
            Some(room) => room,
            None => return,
        };
        for client in room.clients() {
            client.result_addr().do_send(result.clone());
        }
    }

    pub fn send_client_result(&self, client_id: ClientId, result: ResultMessage) {
        let client = match self.sessions.get(&client_id) {
            Some(client) => client,
            None => return,
        };
        client.result_addr().do_send(result);
    }

    pub fn available_rooms(&self) -> Vec<RoomId> {
        self.rooms
            .iter()
            .filter(|(_, room)| room.clients().len() < 2)
            .map(|(id, _)| *id)
            .collect()
    }
}

impl Actor for Lobby {
    type Context = actix::Context<Self>;
}

impl Handler<ConnectMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ConnectMessage, _: &mut Self::Context) -> Self::Result {
        let client = Arc::new(msg.client);
        let client_id = client.id();
        let room_id = msg.room_id;
        let room = match self.rooms.get_mut(&room_id) {
            Some(room) => room,
            None => {
                let room = Room::new(room_id);
                self.rooms.insert(room_id, room);
                self.rooms.get_mut(&room_id).unwrap()
            }
        };

        let player_color = match room.add_client(client.clone()) {
            Ok(color) => color,
            Err(e) => {
                let err = ResultMessage::error(room_id, client_id, e.to_string());
                self.send_client_result(client_id, err);
                return;
            }
        };

        let join_message = ResultMessage::connect(room_id, client_id, player_color);

        self.send_room_result(room_id, join_message);
        self.sessions.insert(client.id(), client);
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
                let error_message = format!("Room {} does not exist", room_id);
                let err = ResultMessage::error(room_id, client_id, error_message);
                self.send_client_result(client_id, err);
                return;
            }
        };
        let client = match self.sessions.get(&client_id) {
            Some(client) => client,
            None => {
                return;
            }
        };

        match room.remove_client(client) {
            Ok(_) => (),
            Err(e) => {
                let err = ResultMessage::error(room_id, client_id, e.to_string());
                self.send_client_result(client_id, err);
                return;
            }
        }
        self.sessions.remove(&client_id);

        if room.clients().is_empty() {
            println!("Room {} is empty, removing", room_id);
            self.rooms.remove(&room_id);
        } else {
            let leave_message = ResultMessage::disconnect(room_id, client_id);

            self.send_room_result(room_id, leave_message);
        }
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
                let error_message = format!("Room {} does not exist", room_id);
                let err = ResultMessage::error(room_id, client_id, error_message);
                self.send_client_result(client_id, err);
                return;
            }
        };

        let result = match command {
            Command::Move { from, to } => room.make_move(client_id, from, to),
            Command::Promote { piece } => room.promote(client_id, piece),
            Command::Resign => room.resign(client_id),
        };

        match result {
            Ok(_) => (),
            Err(e) => {
                let err = ResultMessage::error(room_id, client_id, e.to_string());
                self.send_client_result(client_id, err);
                return;
            }
        }
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
        let room_id = msg.0;
        let room = match self.rooms.get(&room_id) {
            Some(room) => room,
            None => return false,
        };
        room.clients().len() < 2
    }
}
