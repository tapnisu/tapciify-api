mod convert;
mod convert_raw;
mod root;

use axum::{routing::post, Router};
use convert::convert;
use convert_raw::convert_raw;
use root::root;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", post(root))
        .route("/convert", post(convert))
        .route("/convert/raw", post(convert_raw))
}
