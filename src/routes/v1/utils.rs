use std::io::Cursor;

use anyhow::Result;
use axum::{body::Bytes, extract::Query};
use image::{imageops::FilterType, io::Reader as ImageReader};
use serde::Deserialize;
use tapciify::{
    AsciiArt, AsciiArtConverter, AsciiArtConverterOptions, CustomRatioResize, DEFAULT_ASCII_STRING,
    DEFAULT_FONT_RATIO,
};

#[derive(Deserialize, Debug, Clone)]
pub struct ConvertQuery {
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[serde(rename = "asciiString")]
    pub ascii_string: Option<String>,
    #[serde(rename = "fontRatio")]
    pub font_ratio: Option<f64>,
    pub reverse: Option<bool>,
}

pub fn bytes_to_ascii(
    bytes: &Bytes,
    query: &Query<ConvertQuery>,
    colored: bool,
) -> Result<AsciiArt> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    let ascii_string = match query.ascii_string.to_owned() {
        Some(ascii_string) => urlencoding::decode(&ascii_string)?.into_owned(),
        None => DEFAULT_ASCII_STRING.to_owned()
    };

    let ascii_art = img
        .resize_custom_ratio(
            query.width,
            query.height,
            query.font_ratio.unwrap_or(DEFAULT_FONT_RATIO),
            FilterType::Triangle,
        )
        .ascii_art(&AsciiArtConverterOptions {
            ascii_string: match query.reverse.unwrap_or(false) {
                true => ascii_string.chars().rev().collect(),
                false => ascii_string
            },
            colored,
        })?;

    Ok(ascii_art)
}
