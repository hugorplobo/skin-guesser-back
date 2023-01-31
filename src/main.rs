use std::env;

use actix_web::{HttpServer, App, web};
use actix_cors::Cors;
use dotenvy::dotenv;

mod model;
mod routes;
mod services;

use routes::game::game;
use model::database::Database;

pub struct AppState {
    database: Database
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database = Database::new().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(
                AppState { database: database.clone() }
            ))
            .service(game)
    })
    .bind(("127.0.0.1", env::var("PORT")
        .expect("The port is necessary!")
        .parse::<u16>()
        .unwrap()
    ))?
    .run()
    .await
}
