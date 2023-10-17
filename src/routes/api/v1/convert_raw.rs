use super::query::ConvertQuery;
use axum::{
    extract::{Multipart, Query},
    Json,
};
use image::io::Reader as ImageReader;
use serde::Serialize;
use std::io::Cursor;
use tapciify::{AsciiConverter, RawAsciiArt, DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};

#[derive(Serialize)]
pub struct AsciiCharacterDef {
    pub character: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Serialize)]
pub struct RawAsciiArtDef {
    pub characters: Vec<AsciiCharacterDef>,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
pub struct ConvertRawResult {
    pub data: Vec<RawAsciiArtDef>,
}

pub async fn convert_raw(
    query: Query<ConvertQuery>,
    mut multipart: Multipart,
) -> Json<ConvertRawResult> {
    let mut raw_ascii_images: Vec<RawAsciiArt> = vec![];

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

        raw_ascii_images.push(ascii_converter.convert_raw().unwrap());
    }

    Json(ConvertRawResult {
        data: raw_ascii_images
            .iter()
            .map(|raw_ascii_image| RawAsciiArtDef {
                characters: raw_ascii_image
                    .characters
                    .iter()
                    .map(|ascii_character| AsciiCharacterDef {
                        character: ascii_character.character,
                        r: ascii_character.r,
                        g: ascii_character.g,
                        b: ascii_character.b,
                        a: ascii_character.a,
                    })
                    .collect(),
                width: raw_ascii_image.width,
                height: raw_ascii_image.height,
            })
            .collect(),
    })
}
