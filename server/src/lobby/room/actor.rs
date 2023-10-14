use actix::{Actor, Handler};

use crate::{commands::Command, messages::result::ResultMessage};

use super::{
    message::{self, Connect, Disconnect, RoomMessage},
    Room,
};

impl Actor for Room {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_timer(ctx);
    }
}

impl Handler<Connect> for Room {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let client = msg.0;
        let result = self.add_client(client.clone());
        match result {
            Ok(full) => {
                if full {
                    let msg = RoomMessage {
                        room_id: self.id,
                        message: message::RoomMessageType::Full,
                    };
                    self.lobby.do_send(msg);
                    self.start_game();
                }
            }
            Err(e) => {
                let msg = ResultMessage::error(self.id, client.id(), e.to_string());
                client.result_addr().do_send(msg);
            }
        }
    }
}

impl Handler<Disconnect> for Room {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        let client_id = msg.0;
        let result = self.remove_client(client_id);
        match result {
            Ok(empty) => {
                if empty {
                    let msg = RoomMessage {
                        room_id: self.id,
                        message: message::RoomMessageType::Empty,
                    };
                    self.lobby.do_send(msg);
                } else {
                    let msg = RoomMessage {
                        room_id: self.id,
                        message: message::RoomMessageType::Disconnect,
                    };
                    self.lobby.do_send(msg);
                }
            }
            Err(_) => {}
        }
    }
}

impl Handler<message::Command> for Room {
    type Result = ();

    fn handle(&mut self, msg: message::Command, _: &mut Self::Context) -> Self::Result {
        let client_id = msg.client_id;
        let command = msg.command;

        let result = match command {
            Command::Move { from, to } => {
                let result = self.make_move(client_id, from, to);
                result
            }
            Command::Promote { piece } => {
                let result = self.promote(client_id, piece);
                result
            }
            Command::Reset(reset) => {
                if reset {
                    let result = self.reset(client_id);
                    result
                } else {
                    Ok(())
                }
            }
            Command::Resign(resign) => {
                if resign {
                    let result = self.resign(client_id);
                    result
                } else {
                    Ok(())
                }
            }
        };
        let client = self.client(client_id);
        let client = match client {
            Some(client) => client,
            None => {
                return;
            }
        };
        match result {
            Ok(_) => (),
            Err(e) => {
                let msg = ResultMessage::error(self.id, client_id, e.to_string());
                client.result_addr().do_send(msg);
            }
        }
    }
}
