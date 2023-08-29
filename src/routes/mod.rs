mod api;

use api::create_api_routes;
use axum::{http::Method, Router};
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let v1 = api::v1::create_v1_routes();
    let api = create_api_routes();

    Router::new().nest("/", v1).nest("/api", api).layer(cors)
}
