mod convert;
mod convert_raw;
mod root;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use convert::convert;
use convert_raw::convert_raw;
use root::root;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let v1 = Router::new()
        .route("/", get(root))
        .route("/convert", post(convert))
        .route("/convert/raw", post(convert_raw));

    let api = Router::new().nest("/", v1.clone()).nest("/v1", v1.clone());

    Router::new()
        .nest("/", v1.clone())
        .nest("/api", api)
        .layer(cors)
}
