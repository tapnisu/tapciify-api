use super::settings::Settings;
use axum::http::HeaderMap;
use axum::{extract::Multipart, Json};
use image::io::Reader as ImageReader;
use serde::Serialize;
use std::io::Cursor;
use tapciify::{AsciiArt, AsciiConverter};

#[derive(Serialize)]
pub struct AsciiArtDef {
    #[serde(rename = "asciiArt")]
    pub ascii_art: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
pub struct ConvertResult {
    pub data: Vec<AsciiArtDef>,
}

pub async fn convert(headers: HeaderMap, mut multipart: Multipart) -> Json<ConvertResult> {
    let mut raw_ascii_images: Vec<AsciiArt> = vec![];
    let settings = Settings::new(headers);

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        let img = ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let ascii_converter = AsciiConverter {
            img,
            width: settings.width,
            height: settings.height,
            ascii_string: if settings.reverse {
                settings.ascii_string.clone().chars().rev().collect()
            } else {
                settings.ascii_string.clone()
            },
            font_ratio: settings.font_ratio,
            ..Default::default()
        };

        raw_ascii_images.push(ascii_converter.convert().unwrap());
    }

    Json(ConvertResult {
        data: raw_ascii_images
            .iter()
            .map(|raw_ascii_image| AsciiArtDef {
                ascii_art: raw_ascii_image.text.clone(),
                width: raw_ascii_image.width,
                height: raw_ascii_image.height,
            })
            .collect(),
    })
}
