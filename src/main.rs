use axum::{extract::Multipart, routing::post, Json, Router};
use image::io::Reader as ImageReader;
use serde::Serialize;
use std::io::Cursor;
use tapciify::{AsciiConverter, AsciiImage, RawAsciiImage};

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
    pub result: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
pub struct RawAsciiImageDef {
    pub result: Vec<AsciiCharacterDef>,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
struct ConvertResult {
    result: Vec<AsciiImageDef>,
}

#[derive(Serialize)]
struct ConvertColoredResult {
    result: Vec<RawAsciiImageDef>,
}

async fn root() {}

async fn convert(mut multipart: Multipart) -> Json<ConvertResult> {
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
            width: 64,
            ..Default::default()
        };

        raw_ascii_images.push(ascii_converter.convert());
    }

    Json(ConvertResult {
        result: raw_ascii_images
            .iter()
            .map(|raw_ascii_image| AsciiImageDef {
                result: raw_ascii_image.result.clone(),
                width: raw_ascii_image.width,
                height: raw_ascii_image.height,
            })
            .collect(),
    })
}

async fn convert_colored(mut multipart: Multipart) -> Json<ConvertColoredResult> {
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
            width: 64,
            ..Default::default()
        };

        raw_ascii_images.push(ascii_converter.convert_raw());
    }

    Json(ConvertColoredResult {
        result: raw_ascii_images
            .iter()
            .map(|raw_ascii_image| RawAsciiImageDef {
                result: raw_ascii_image
                    .result
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
