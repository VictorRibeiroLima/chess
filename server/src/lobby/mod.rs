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
        success::{ConnectSuccess, DisconnectSuccess, SuccessMessage, SuccessResult},
        AvailableRoom, AvailableRooms, CommandMessage, ConnectMessage, DisconnectMessage,
        ErrorMessage, StringMessage,
    },
};

use self::{client::Client, room::Room};

#[derive(Default, Clone)]
pub struct Lobby {
    sessions: HashMap<ClientId, Arc<Client>>,
    rooms: HashMap<RoomId, Room>,
}

impl Lobby {
    pub fn send_room_success(&self, room: RoomId, result: SuccessMessage) -> Option<()> {
        let room = self.rooms.get(&room)?;
        for client in room.clients() {
            client.success_addr().do_send(result.clone());
        }
        Some(())
    }

    pub fn _send_client_success(&self, client_id: ClientId, result: SuccessMessage) -> Option<()> {
        let client = self.sessions.get(&client_id)?;
        client.success_addr().do_send(result);
        Some(())
    }

    pub fn send_client_error(&self, err: ErrorMessage) -> Option<()> {
        let client = self.sessions.get(&err.client_id)?;
        client.error_addr().do_send(err);
        Some(())
    }

    pub fn _send_all_message(&self, msg: &str) {
        for client in self.sessions.values() {
            client._string_addr().do_send(StringMessage(msg.to_owned()));
        }
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
                let err = ErrorMessage {
                    client_id,
                    room_id,
                    error: e.to_string(),
                };
                self.send_client_error(err);
                return;
            }
        };

        let join_message = SuccessMessage {
            client_id,
            room_id,
            result: SuccessResult::Connect(ConnectSuccess {
                client_id,
                room_id,
                color: player_color,
            }),
        };

        self.send_room_success(room_id, join_message);
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
                let err = ErrorMessage {
                    client_id,
                    room_id,
                    error: error_message,
                };
                self.send_client_error(err);
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
                let err = ErrorMessage {
                    client_id,
                    room_id,
                    error: e.to_string(),
                };
                self.send_client_error(err);
                return;
            }
        }
        self.sessions.remove(&client_id);

        if room.clients().is_empty() {
            println!("Room {} is empty, removing", room_id);
            self.rooms.remove(&room_id);
        } else {
            let leave_message = SuccessMessage {
                client_id,
                room_id,
                result: SuccessResult::Disconnect(DisconnectSuccess { client_id, room_id }),
            };

            self.send_room_success(room_id, leave_message);
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
                let err = ErrorMessage {
                    client_id,
                    room_id,
                    error: error_message,
                };
                self.send_client_error(err);
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
                let err = ErrorMessage {
                    client_id,
                    room_id,
                    error: e.to_string(),
                };
                self.send_client_error(err);
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
