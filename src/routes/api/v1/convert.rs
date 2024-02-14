use super::query::ConvertQuery;
use axum::{
    extract::{Multipart, Query},
    Json,
};
use image::{imageops::FilterType, io::Reader as ImageReader};
use serde::Serialize;
use std::io::Cursor;
use tapciify::{
    AsciiArt, AsciiArtConverter, AsciiArtConverterOptions, CustomRatioResize, DEFAULT_ASCII_STRING,
    DEFAULT_FONT_RATIO,
};

#[derive(Serialize, Debug, Clone)]
pub struct AsciiArtDef {
    #[serde(rename = "asciiArt")]
    pub ascii_art: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Debug, Clone)]
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
            .to_owned()
            .map_or(DEFAULT_ASCII_STRING.to_owned(), |encoded| {
                urlencoding::decode(&encoded).unwrap().into_owned()
            });

        let ascii_art = img
            .resize_custom_ratio(
                query.width,
                query.height,
                query.font_ratio.unwrap_or(DEFAULT_FONT_RATIO),
                FilterType::Triangle,
            )
            .ascii_art(&AsciiArtConverterOptions {
                ascii_string: if query.reverse.unwrap_or(false) {
                    ascii_string.chars().rev().collect()
                } else {
                    ascii_string
                },
                colored: true,
            })
            .unwrap();

        ascii_image.push(ascii_art);
    }

    Json(ConvertResult {
        data: ascii_image
            .iter()
            .map(|raw_ascii_image| AsciiArtDef {
                ascii_art: raw_ascii_image.to_string(),
                width: raw_ascii_image.width,
                height: raw_ascii_image.height,
            })
            .collect(),
    })
}
