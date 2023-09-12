mod api;

use api::{create_api_routes, v1::create_v1_routes};
use axum::{http::Method, response::Redirect, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let v1 = create_v1_routes();
    let api = create_api_routes();

    Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("https://github.com/tapciify/api") }),
        )
        .nest("/", v1)
        .nest("/api", api)
        .layer(cors)
}
