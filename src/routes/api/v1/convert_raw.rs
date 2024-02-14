use super::query::ConvertQuery;
use axum::{
    extract::{Multipart, Query},
    Json,
};
use image::{imageops::FilterType, io::Reader as ImageReader};
use serde::Serialize;
use std::io::Cursor;
use tapciify::{
    AsciiArt, AsciiArtConverter, AsciiArtConverterOptions, AsciiArtPixel, CustomRatioResize,
    DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO,
};

#[derive(Serialize, Debug, Clone)]
pub struct AsciiCharacterDef {
    pub character: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<AsciiArtPixel> for AsciiCharacterDef {
    fn from(p: AsciiArtPixel) -> AsciiCharacterDef {
        AsciiCharacterDef {
            character: p.character,
            r: p.r,
            g: p.g,
            b: p.b,
            a: p.a,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct RawAsciiArtDef {
    pub characters: Vec<AsciiCharacterDef>,
    pub width: u32,
    pub height: u32,
}

impl From<AsciiArt> for RawAsciiArtDef {
    fn from(a: AsciiArt) -> RawAsciiArtDef {
        RawAsciiArtDef {
            characters: a.characters.iter().map(|c| c.to_owned().into()).collect(),
            width: a.width,
            height: a.height,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ConvertRawResult {
    pub data: Vec<RawAsciiArtDef>,
}

pub async fn convert_raw(
    query: Query<ConvertQuery>,
    mut multipart: Multipart,
) -> Json<ConvertRawResult> {
    let mut ascii_arts: Vec<AsciiArt> = vec![];

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

        ascii_arts.push(ascii_art);
    }

    Json(ConvertRawResult {
        data: ascii_arts
            .iter()
            .map(|ascii_art| ascii_art.to_owned().into())
            .collect(),
    })
}
