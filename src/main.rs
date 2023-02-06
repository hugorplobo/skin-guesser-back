use actix_web::{HttpServer, App, web};
use actix_cors::Cors;
use dotenvy::dotenv;

mod model;
mod routes;
mod services;

use routes::{game::game, new_game::new_game};
use model::database::Database;
use services::create_game::create_game;
use tokio_cron_scheduler::{JobScheduler, Job};

pub struct AppState {
    database: Database
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let sched = JobScheduler::new().await.unwrap();
    sched.add(Job::new_async("@daily", |_, _| Box::pin(async {
        create_game().await;
    })).unwrap()).await.unwrap();

    sched.start().await.unwrap();

    let database = Database::new().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(
                AppState { database: database.clone() }
            ))
            .service(game)
            .service(new_game)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
