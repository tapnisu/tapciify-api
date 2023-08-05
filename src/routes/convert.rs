use crate::structs::{AsciiArtDef, ConvertQuery, ConvertResult};
use axum::{
    extract::{Multipart, Query},
    Json,
};
use image::io::Reader as ImageReader;
use std::io::Cursor;
use tapciify::{AsciiArt, AsciiConverter, DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};

pub async fn convert(query: Query<ConvertQuery>, mut multipart: Multipart) -> Json<ConvertResult> {
    let mut raw_ascii_images: Vec<AsciiArt> = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        let img = ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let ascii_converter = AsciiConverter {
            img,
            width: query.width.unwrap_or(0),
            height: query.height.unwrap_or(0),
            ascii_string: query
                .ascii_string
                .clone()
                .unwrap_or(DEFAULT_ASCII_STRING.to_owned()),
            font_ratio: query.font_ratio.unwrap_or(DEFAULT_FONT_RATIO),
            ..Default::default()
        };

        raw_ascii_images.push(ascii_converter.convert());
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
