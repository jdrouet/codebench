mod config;
mod controller;

use actix_web::{App, HttpServer};
use clap::Parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::parse();

    HttpServer::new(|| App::new().service(controller::status::handler))
        .bind(config.server_address())?
        .run()
        .await
}
