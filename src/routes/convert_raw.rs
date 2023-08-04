use crate::structs::{AsciiCharacterDef, ConvertQuery, ConvertRawResult, RawAsciiImageDef};
use axum::{
    extract::{Multipart, Query},
    Json,
};
use image::io::Reader as ImageReader;
use std::io::Cursor;
use tapciify::{AsciiConverter, RawAsciiArt};

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

        let ascii_converter = AsciiConverter {
            img,
            width: query.width.unwrap_or(0),
            height: query.height.unwrap_or(0),
            ..Default::default()
        };

        raw_ascii_images.push(ascii_converter.convert_raw());
    }

    Json(ConvertRawResult {
        data: raw_ascii_images
            .iter()
            .map(|raw_ascii_image| RawAsciiImageDef {
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