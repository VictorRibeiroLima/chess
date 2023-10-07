use std::time::Duration;

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{
    get,
    middleware::NormalizePath,
    web::{Data, Path, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use con::Con;
use lobby::Lobby;
use messages::inner::{AvailableRoom, AvailableRooms};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

mod commands;
mod con;
mod lobby;
mod messages;

#[get("api")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello chess world (cached)!")
}

#[get("api/room")]
async fn available_rooms(addr: Data<Addr<Lobby>>) -> impl Responder {
    let rooms = addr.get_ref().send(AvailableRooms).await;
    match rooms {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("ws/room/create")]
async fn create_room(
    req: HttpRequest,
    stream: Payload,
    addr: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let group_id = Uuid::new_v4();
    let addr = addr.get_ref().clone();
    let ws = Con::new(group_id, addr);

    let resp = actix_web_actors::ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[get("ws/room/{room_id}")]
async fn join_room(
    room_id: Path<Uuid>,
    req: HttpRequest,
    stream: Payload,
    addr: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let room_id = room_id.into_inner();
    let addr = addr.get_ref().clone();

    let room = match addr.send(AvailableRoom(room_id)).await {
        Ok(room) => room,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    if !room {
        return Ok(HttpResponse::NotFound().finish());
    }

    let ws = Con::new(room_id, addr);

    let resp = actix_web_actors::ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let lobby = Lobby::default();
    let addr = lobby.start();

    println!("Server running at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Cors::permissive())
            .app_data(Data::new(addr.clone()))
            .service(hello)
            .service(available_rooms)
            .service(create_room)
            .service(join_room)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
