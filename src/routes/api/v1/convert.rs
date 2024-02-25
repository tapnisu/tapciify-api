use super::utils::{bytes_to_ascii, ConvertQuery};
use axum::{
    extract::{Multipart, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tapciify::AsciiArt;

#[derive(Serialize, Debug, Clone)]
pub struct AsciiArtDef {
    #[serde(rename = "asciiArt")]
    pub ascii_art: String,
    pub width: u32,
    pub height: u32,
}

impl From<AsciiArt> for AsciiArtDef {
    fn from(a: AsciiArt) -> AsciiArtDef {
        AsciiArtDef {
            ascii_art: a.to_string(),
            width: a.width,
            height: a.height,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ConvertResult {
    pub data: Vec<AsciiArtDef>,
}

pub async fn convert(query: Query<ConvertQuery>, mut multipart: Multipart) -> Response {
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
                return (
                    StatusCode::BAD_REQUEST,
                    format!("Reading image error: {}", e),
                )
                    .into_response()
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

    let body = Json(ConvertResult { data });

    (StatusCode::OK, body).into_response()
}
