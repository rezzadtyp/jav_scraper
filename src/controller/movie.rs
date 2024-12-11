use crate::service::post_123av as movie_details;
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ScrapeRequest {
    pub url: String,
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/scrape").route(web::post().to(scrape_movie_details)));
}

async fn scrape_movie_details(
    data: web::Json<ScrapeRequest>,
    client: web::Data<Arc<Client>>,
) -> impl Responder {
    match movie_details::fetch_movie_details(&data.url, &client).await {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
