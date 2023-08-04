mod convert;
mod convert_raw;
mod root;

use axum::{http::Method, routing::post, Router};
use convert::convert;
use convert_raw::convert_raw;
use root::root;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", post(root))
        .route("/convert", post(convert))
        .route("/convert/raw", post(convert_raw))
        .layer(cors)
}
