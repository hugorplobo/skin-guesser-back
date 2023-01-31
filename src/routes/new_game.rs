use actix_web::{Responder, get, HttpResponse};

use crate::services::create_game::create_game;

#[get("/new")]
pub async fn new_game() -> impl Responder {
    create_game().await;
    HttpResponse::Ok()
}