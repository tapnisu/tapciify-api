mod convert;
mod convert_raw;

use axum::{routing::post, Router};
use convert::convert;
use convert_raw::convert_raw;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConvertQuery {
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[serde(rename = "asciiString")]
    pub ascii_string: Option<String>,
    #[serde(rename = "fontRatio")]
    pub font_ratio: Option<f64>,
    pub reverse: Option<bool>,
}

pub fn create_v2_routes() -> Router {
    Router::new()
        .route("/convert", post(convert))
        .route("/convert/raw", post(convert_raw))
}
