use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ConvertQuery {
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[serde(rename = "asciiString")]
    pub ascii_string: Option<String>,
    #[serde(rename = "fontRatio")]
    pub font_ratio: Option<f64>,
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
pub struct AsciiArtDef {
    #[serde(rename = "asciiArt")]
    pub ascii_art: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
pub struct RawAsciiArtDef {
    pub characters: Vec<AsciiCharacterDef>,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
pub struct ConvertResult {
    pub data: Vec<AsciiArtDef>,
}

#[derive(Serialize)]
pub struct ConvertRawResult {
    pub data: Vec<RawAsciiArtDef>,
}
