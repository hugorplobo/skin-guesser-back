use chrono::NaiveDate;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Game {
    pub id: i32,
    pub time: NaiveDate,
    pub pt_skin_name: String,
    pub en_skin_name: String,
    pub base64_img: String
}