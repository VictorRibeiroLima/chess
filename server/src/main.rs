use std::time::Duration;

use actix_cors::Cors;
use actix_web::{get, middleware::NormalizePath, App, HttpResponse, HttpServer, Responder};
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Cors::permissive())
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
