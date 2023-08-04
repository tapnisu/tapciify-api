use serde::{Deserialize, Serialize};

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
pub struct ConvertResult {
    pub data: Vec<AsciiImageDef>,
}

#[derive(Serialize)]
pub struct ConvertRawResult {
    pub data: Vec<RawAsciiImageDef>,
}
