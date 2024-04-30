use axum::{http::Method, response::Redirect, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

use v1::create_v1_routes;

mod v1;

pub fn create_routes() -> Router {
    let v1_routes = create_v1_routes();

    Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("https://github.com/tapciify/api") }),
        )
        .nest("/", v1_routes.to_owned())
        .nest("/v1", v1_routes.to_owned())
        .nest("/api/v1", v1_routes)
}
