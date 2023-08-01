use axum::{
    extract::{Multipart, Query},
    routing::post,
    Json, Router,
};
use image::io::Reader as ImageReader;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tapciify::{AsciiConverter, AsciiImage, RawAsciiImage};

#[derive(Deserialize)]
pub struct ConvertQuery {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Serialize)]
pub struct AsciiCharacterDef {
    pub character: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Serialize)]
pub struct AsciiImageDef {
    #[serde(rename = "asciiImage")]
    pub ascii_image: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
pub struct RawAsciiImageDef {
    pub characters: Vec<AsciiCharacterDef>,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
struct ConvertResult {
    data: Vec<AsciiImageDef>,
}

#[derive(Serialize)]
struct ConvertColoredResult {
    data: Vec<RawAsciiImageDef>,
}

async fn root() {}

async fn convert(query: Query<ConvertQuery>, mut multipart: Multipart) -> Json<ConvertResult> {
    let mut raw_ascii_images: Vec<AsciiImage> = vec![];

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

        raw_ascii_images.push(ascii_converter.convert());
    }

    Json(ConvertResult {
        data: raw_ascii_images
            .iter()
            .map(|raw_ascii_image| AsciiImageDef {
                ascii_image: raw_ascii_image.text.clone(),
                width: raw_ascii_image.width,
                height: raw_ascii_image.height,
            })
            .collect(),
    })
}

async fn convert_colored(
    query: Query<ConvertQuery>,
    mut multipart: Multipart,
) -> Json<ConvertColoredResult> {
    let mut raw_ascii_images: Vec<RawAsciiImage> = vec![];

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

    Json(ConvertColoredResult {
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

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .route("/", post(root))
        .route("/convert", post(convert))
        .route("/convert/colored", post(convert_colored));

    Ok(app.into())
}
