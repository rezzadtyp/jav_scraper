mod service;
mod controller;

use actix_web::{web, App, HttpServer};
use reqwest::Client;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(Client::new());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(controller::movie::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}