use chrono::Utc;
use serde_json::Value;

use crate::model::{database::Database, game::Game};
use rand::prelude::*;

use super::image::{download_image, resize_image, encode_img_to_base64};

pub async fn create_game() {
    let database = Database::new_single().await;
    let champion = get_random_champion().await.unwrap();
    let (skin_name, skin_num) = get_random_skin(&champion).await.unwrap();

    let img = download_image(
        &format!("{champion}_{skin_num}")
    ).await;
    let img = resize_image(img);
    let base64 = encode_img_to_base64(img);
    let game = Game {
        id: 0,
        time: Utc::now().date_naive(),
        en_skin_name: String::from(""),
        pt_skin_name: skin_name,
        base64_img: base64
    };

    database.insert_game(&game).await.unwrap();
}

async fn get_random_champion() -> Result<String, reqwest::Error> {
    let url = "http://ddragon.leagueoflegends.com/cdn/13.1.1/data/pt_BR/champion.json";
    let res = reqwest::get(url)
        .await?
        .text()
        .await?;
    
    let json: Value = serde_json::from_str(&res).unwrap();

    let array: Vec<_> = json["data"]
        .as_object()
        .unwrap()
        .iter()
        .map(|x| x.0)
        .collect();
    
    let mut rng = thread_rng();
    let index = rng.gen_range(0..array.len());
    
    Ok(array[index].clone())
}

async fn get_random_skin(champion: &str) -> Result<(String, i64), reqwest::Error> {
    let url = format!("http://ddragon.leagueoflegends.com/cdn/13.1.1/data/pt_BR/champion/{champion}.json");
    let res = reqwest::get(url)
        .await?
        .text()
        .await?;
    
    let json: Value = serde_json::from_str(&res).unwrap();
    let array: Vec<_> = json["data"][champion]["skins"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| (x["name"].as_str().unwrap().to_string(), x["num"].as_i64().unwrap()))
        .collect();
    
    let mut rng = thread_rng();
    let index = rng.gen_range(0..array.len());
    let skin =  if array[index].0 == "default" {
        (champion.to_string(), array[index].1)
    } else {
        array[index].clone()
    };

    Ok(skin)
}