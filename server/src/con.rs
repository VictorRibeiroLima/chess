use std::time::Instant;

use crate::{
    commands,
    lobby::{client::Client, Lobby},
    messages::{
        success::SuccessMessage, CommandMessage, ConnectMessage, DisconnectMessage, ErrorMessage,
        StringMessage,
    },
    CLIENT_TIMEOUT, HEARTBEAT_INTERVAL,
};
use actix::{
    fut, prelude::ContextFutureSpawner, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
    Handler, Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use uuid::Uuid;

pub struct Con {
    id: Uuid,
    room: Uuid,
    lobby_addr: Addr<Lobby>,
    heartbeat: Instant,
}

impl Con {
    pub fn new(room: Uuid, lobby: Addr<Lobby>) -> Con {
        Con {
            id: Uuid::new_v4(),
            room,
            heartbeat: Instant::now(),
            lobby_addr: lobby,
        }
    }

    pub fn start_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for Con {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);

        let addr = ctx.address();

        let client = Client::new(self.id, addr);

        self.lobby_addr
            .send(ConnectMessage {
                room_id: self.room,
                client,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(DisconnectMessage {
            room_id: self.room,
            client_id: self.id,
        });

        Running::Stop
    }
}

//Receive message from lobby and send to client
impl Handler<StringMessage> for Con {
    type Result = ();

    fn handle(&mut self, msg: StringMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl Handler<ErrorMessage> for Con {
    type Result = ();

    fn handle(&mut self, msg: ErrorMessage, ctx: &mut Self::Context) -> Self::Result {
        if msg.client_id == self.id {
            ctx.text(msg.error);
        }
    }
}

impl Handler<SuccessMessage> for Con {
    type Result = ();

    fn handle(&mut self, msg: SuccessMessage, ctx: &mut Self::Context) -> Self::Result {
        let msg = serde_json::to_string(&msg).unwrap();
        ctx.text(msg);
    }
}

//Receive message from client and send to lobby
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Con {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let text = text.trim();
                let command = serde_json::from_str::<commands::Command>(text);
                let command = match command {
                    Ok(command) => command,
                    Err(_) => {
                        let err = commands::Error::InvalidCommand;
                        let err = ErrorMessage {
                            client_id: self.id,
                            room_id: self.room,
                            error: err.to_string(),
                        };
                        let err = serde_json::to_string(&err).unwrap();
                        ctx.text(err);
                        return;
                    }
                };

                let command_message = CommandMessage {
                    room_id: self.room,
                    client_id: self.id,
                    command,
                };

                self.lobby_addr
                    .send(command_message)
                    .into_actor(self)
                    .then(|res, _, ctx| {
                        match res {
                            Ok(_) => (),
                            _ => ctx.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
