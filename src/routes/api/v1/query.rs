use serde::Deserialize;

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
