use actix_web::{Responder, HttpResponse, get, web};
use serde::{Deserialize, Serialize};

use crate::{AppState, services::hash::hash};

#[derive(Deserialize)]
pub struct QueryData {
    date: String
}

#[derive(Serialize)]
pub struct Response {
    base64_img: String,
    skin_name: String
}

#[get("/game")]
pub async fn game(data: web::Query<QueryData>, state: web::Data<AppState>) -> impl Responder {
    if let Ok(game) = state.database.get_game_by_date(&data.date).await {
        HttpResponse::Ok().json(Response {
            base64_img: game.base64_img,
            skin_name: hash(&game.pt_skin_name)
        })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}