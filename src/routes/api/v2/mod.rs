mod convert;
mod convert_raw;

use axum::{routing::post, Router};
use convert::convert;
use convert_raw::convert_raw;

pub fn create_v2_routes() -> Router {
    Router::new()
        .route("/convert", post(convert))
        .route("/convert/raw", post(convert_raw))
}
