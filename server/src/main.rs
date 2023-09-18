use std::{sync::Arc, time::Duration};

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{
    get,
    middleware::NormalizePath,
    web::{Data, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use con::Con;
use lobby::Lobby;
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

type ClientId = Uuid;
type RoomId = Uuid;

mod client;
mod con;
mod lobby;
mod messages;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/create")]
async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Arc<Addr<Lobby>>>,
) -> Result<HttpResponse, Error> {
    let group_id = Uuid::new_v4();
    let addr = srv.get_ref().as_ref().clone();
    let ws = Con::new(group_id, addr);

    let resp = actix_web_actors::ws::start(ws, &req, stream)?;
    println!("Response: {:?}", resp);
    Ok(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let lobby = lobby::Lobby::default().start();
    println!("Starting server at");
    let lobby = Arc::new(lobby);
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Cors::permissive())
            .app_data(Data::new(lobby.clone()))
            .service(hello)
            .service(start_connection)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
