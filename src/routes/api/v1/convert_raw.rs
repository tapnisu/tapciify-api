use super::utils::{bytes_to_ascii, ConvertQuery};
use axum::{
    extract::{Multipart, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tapciify::{AsciiArt, AsciiArtPixel};

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

pub async fn convert_raw(query: Query<ConvertQuery>, mut multipart: Multipart) -> Response {
    let mut ascii_arts: Vec<AsciiArt> = vec![];

    while let Some(field) = match multipart.next_field().await {
        Ok(fields) => fields,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, format!("Multipart error: {}", e)).into_response()
        }
    } {
        let bytes = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => {
                return (StatusCode::BAD_REQUEST, format!("Reading image error: {}", e)).into_response()
            }
        };

        let ascii_art = match bytes_to_ascii(&bytes, &query) {
            Ok(ascii_art) => ascii_art,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    format!("ASCII art conversion error: {}", e),
                )
                    .into_response()
            }
        };

        ascii_arts.push(ascii_art);
    }

    let data = ascii_arts
        .iter()
        .map(|ascii_art| ascii_art.to_owned().into())
        .collect();

    let body = Json(ConvertRawResult { data });

    (StatusCode::OK, body).into_response()
}
