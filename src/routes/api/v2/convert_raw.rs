use super::settings::Settings;
use axum::{extract::Multipart, http::HeaderMap, Json};
use image::io::Reader as ImageReader;
use serde::Serialize;
use std::io::Cursor;
use tapciify::{AsciiConverter, RawAsciiArt};

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

pub async fn convert_raw(headers: HeaderMap, mut multipart: Multipart) -> Json<ConvertRawResult> {
    let mut raw_ascii_images: Vec<RawAsciiArt> = vec![];
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
