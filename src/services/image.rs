use std::io::Cursor;

use base64::{engine, Engine};
use image::{DynamicImage, imageops::FilterType};

pub async fn download_image(name: &str) -> DynamicImage {
    let img_bytes = reqwest::get(
        format!("http://ddragon.leagueoflegends.com/cdn/img/champion/loading/{name}.jpg")
    )
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    
    let image = image::load_from_memory(&img_bytes)
        .unwrap();
    
    image
}

pub fn resize_image(img: DynamicImage) -> DynamicImage {
    let resize_factor = 1.5;
    let img = img.resize(
        (img.width() as f64 / resize_factor) as u32,
        (img.height() as f64 / resize_factor) as u32, 
        FilterType::Lanczos3
    );

    img
}

pub fn encode_img_to_base64(img: DynamicImage) -> String {
    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut buffer),
        image::ImageOutputFormat::Png
    ).unwrap();

    engine::general_purpose::STANDARD.encode(buffer)
}