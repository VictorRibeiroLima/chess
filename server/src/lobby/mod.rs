use std::{collections::HashMap, sync::Arc};

use actix::{Actor, Handler};
use uuid::Uuid;

pub mod client;
mod errors;
pub mod room;

pub type ClientId = Uuid;
pub type RoomId = Uuid;

use crate::messages::{ConnectMessage, DisconnectMessage, StringMessage};

use self::{client::Client, room::Room};

#[derive(Default, Clone)]
pub struct Lobby {
    sessions: HashMap<ClientId, Arc<Client>>,
    rooms: HashMap<RoomId, Room>,
}

impl Lobby {
    pub fn send_room_message(&self, room: RoomId, msg: &str) -> Option<()> {
        let room = self.rooms.get(&room)?;
        for client in room.clients() {
            client.addr().do_send(StringMessage(msg.to_owned()));
        }
        Some(())
    }

    pub fn send_client_message(&self, client: ClientId, msg: &str) -> Option<()> {
        let client = self.sessions.get(&client)?;
        client.addr().do_send(StringMessage(msg.to_owned()));
        Some(())
    }

    pub fn _send_all_message(&self, msg: &str) {
        for client in self.sessions.values() {
            client.addr().do_send(StringMessage(msg.to_owned()));
        }
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
                let error_message = format!("Error: {}", e);
                self.send_client_message(client_id, &error_message);
                return;
            }
        };

        let join_message = format!(
            "{} has joined the room, playing has {}",
            client.id(),
            player_color
        );
        self.send_room_message(room_id, &join_message);
        self.sessions.insert(client.id(), client);

        let id_message = format!(
            "Your id is {}, you are playing as {}",
            client_id, player_color
        );
        self.send_client_message(client_id, &id_message);
    }
}

impl Handler<DisconnectMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: DisconnectMessage, _: &mut Self::Context) -> Self::Result {
        let room_id = msg.room_id;
        let client_id = msg.client_id;
        let room = self.rooms.get_mut(&room_id).unwrap();
        let client = self.sessions.get(&client_id).unwrap();

        match room.remove_client(client) {
            Ok(_) => (),
            Err(e) => {
                let error_message = format!("Error: {}", e);
                self.send_client_message(client_id, &error_message);
                return;
            }
        }
        self.sessions.remove(&client_id);

        let leave_message = format!("{} has left the room", client_id);
        self.send_room_message(room_id, &leave_message);
    }
}
