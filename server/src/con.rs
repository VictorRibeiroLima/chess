use std::time::Instant;

use crate::{
    lobby::{client::Client, Lobby},
    messages::{ConnectMessage, DisconnectMessage, StringMessage},
    CLIENT_TIMEOUT, HEARTBEAT_INTERVAL,
};
use actix::{
    fut, prelude::ContextFutureSpawner, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
    Handler, Recipient, StreamHandler, WrapFuture,
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
        let string_addr: Recipient<StringMessage> = addr.recipient();

        let client = Client::new(self.id, string_addr);

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

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.lobby_addr
            .send(DisconnectMessage {
                room_id: self.room,
                client_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, _| {
                match res {
                    Ok(_) => (),
                    _ => (),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

//Receive message from lobby and send to client
impl Handler<StringMessage> for Con {
    type Result = ();

    fn handle(&mut self, msg: StringMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
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
                println!("Text: {}", text);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
