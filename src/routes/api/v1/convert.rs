use super::query::ConvertQuery;
use axum::{
    extract::{Multipart, Query},
    Json,
};
use image::io::Reader as ImageReader;
use serde::Serialize;
use std::io::Cursor;
use tapciify::{AsciiArt, AsciiConverter, DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};

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

pub async fn convert(query: Query<ConvertQuery>, mut multipart: Multipart) -> Json<ConvertResult> {
    let mut ascii_image: Vec<AsciiArt> = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        let img = ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let ascii_string = query
            .ascii_string
            .clone()
            .map_or(DEFAULT_ASCII_STRING.to_owned(), |encoded| {
                urlencoding::decode(&encoded).unwrap().into_owned()
            });

        let ascii_converter = AsciiConverter {
            img,
            width: query.width.unwrap_or(0),
            height: query.height.unwrap_or(0),
            ascii_string: if query.reverse.unwrap_or(false) {
                ascii_string.chars().rev().collect()
            } else {
                ascii_string
            },
            font_ratio: query.font_ratio.unwrap_or(DEFAULT_FONT_RATIO),
            ..Default::default()
        };

        ascii_image.push(ascii_converter.convert().unwrap());
    }

    Json(ConvertResult {
        data: ascii_image
            .iter()
            .map(|raw_ascii_image| AsciiArtDef {
                ascii_art: raw_ascii_image.text.clone(),
                width: raw_ascii_image.width,
                height: raw_ascii_image.height,
            })
            .collect(),
    })
}
