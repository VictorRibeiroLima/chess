use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use actix::{Actor, Handler};

use crate::{
    client::Client,
    messages::{ConnectMessage, DisconnectMessage, StringMessage},
    ClientId, RoomId,
};

#[derive(Default)]
pub struct Lobby {
    sessions: HashMap<ClientId, Rc<Client>>,
    rooms: HashMap<RoomId, HashSet<Rc<Client>>>,
}

impl Lobby {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_room_message(&self, room: RoomId, msg: &str) -> Option<()> {
        let room = self.rooms.get(&room)?;
        for client in room {
            client.addr().do_send(StringMessage(msg.to_owned()));
        }
        Some(())
    }

    pub fn send_client_message(&self, client: ClientId, msg: &str) -> Option<()> {
        let client = self.sessions.get(&client)?;
        client.addr().do_send(StringMessage(msg.to_owned()));
        Some(())
    }

    pub fn send_all_message(&self, msg: &str) {
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
        let client = Rc::new(msg.client);
        let client_id = client.id();
        let room_id = msg.room_id;
        let room = self.rooms.get_mut(&room_id).unwrap();

        room.insert(client.clone());

        let join_message = format!("{} has joined the room", client.id());
        self.send_room_message(room_id, &join_message);
        self.sessions.insert(client.id(), client);

        let id_message = format!("Your id is {}", client_id);
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

        room.remove(client);
        self.sessions.remove(&client_id);

        let leave_message = format!("{} has left the room", client_id);
        self.send_room_message(room_id, &leave_message);
    }
}
