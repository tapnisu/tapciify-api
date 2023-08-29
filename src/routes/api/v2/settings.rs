use axum::http::HeaderMap;
use tapciify::{DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};

pub struct Settings {
    pub width: u32,
    pub height: u32,
    pub ascii_string: String,
    pub font_ratio: f64,
    pub reverse: bool,
}

impl Settings {
    pub fn new(headers: HeaderMap) -> Self {
        let width: u32 = headers["width"].to_str().unwrap_or("0").parse().unwrap();
        let height: u32 = headers["height"].to_str().unwrap_or("0").parse().unwrap();
        let ascii_string: String = headers["ascii-string"]
            .to_str()
            .unwrap_or(DEFAULT_ASCII_STRING)
            .parse()
            .unwrap();
        let font_ratio: f64 = headers["font-ratio"]
            .to_str()
            .unwrap_or(&DEFAULT_FONT_RATIO.to_string())
            .parse()
            .unwrap();
        let reverse: bool = headers["reverse"]
            .to_str()
            .unwrap_or("false")
            .parse()
            .unwrap();

        Self {
            width,
            height,
            ascii_string,
            font_ratio,
            reverse,
        }
    }
}
