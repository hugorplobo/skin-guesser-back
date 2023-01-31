use std::env;

use chrono::NaiveDate;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, PgPool};

use super::game::Game;

#[derive(Clone)]
pub struct Database {
    pub connection: Pool<Postgres>
}

impl Database {
    pub async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(15)
            .connect(&env::var("DATABASE_URL").expect("The database url is necessary!"))
            .await
            .unwrap();

        Database {
            connection: pool
        }
    }

    pub async fn new_single() -> Self {
        let connection = PgPool::connect(
            &env::var("DATABASE_URL").expect("The database url is necessary!")
        ).await.unwrap();

        Database {
            connection
        }
    }

    pub async fn get_game_by_date(&self, date: &str) -> Result<Game, sqlx::Error> {
        let res = sqlx::query_as::<_, Game>("select * from games where time = $1")
            .bind(NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap())
            .fetch_one(&self.connection)
            .await?;
        
        Ok(res)
    }

    pub async fn insert_game(&self, game: &Game) -> Result<(), sqlx::Error> {
        sqlx::query("
            insert into games (time, pt_skin_name, en_skin_name, base64_img) values
            ($1, $2, $3, $4)
        ")
            .bind(game.time)
            .bind(&game.pt_skin_name)
            .bind(&game.en_skin_name)
            .bind(&game.base64_img)
            .execute(&self.connection)
            .await?;

        Ok(())
    }
}