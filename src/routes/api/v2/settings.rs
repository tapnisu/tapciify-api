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
        let width: u32 = if headers.contains_key("width") {
            headers["width"].to_str().unwrap().parse().unwrap()
        } else {
            0
        };

        let height: u32 = if headers.contains_key("height") {
            headers["height"].to_str().unwrap().parse().unwrap()
        } else {
            0
        };

        let ascii_string: String = if headers.contains_key("ascii-string") {
            headers["ascii-string"].to_str().unwrap().parse().unwrap()
        } else {
            DEFAULT_ASCII_STRING.to_owned()
        };

        let font_ratio: f64 = if headers.contains_key("font-ratio") {
            headers["font-ratio"].to_str().unwrap().parse().unwrap()
        } else {
            DEFAULT_FONT_RATIO
        };

        let reverse: bool = if headers.contains_key("reverse") {
            headers["reverse"].to_str().unwrap().parse().unwrap()
        } else {
            false
        };

        Self {
            width,
            height,
            ascii_string,
            font_ratio,
            reverse,
        }
    }
}
