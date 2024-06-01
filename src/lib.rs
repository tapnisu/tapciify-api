use axum::{
    http::Method,
    response::{Html, Redirect},
    routing::get,
    Router,
};
use axum_swagger_ui::swagger_ui;
use tower_http::cors;
use v1::create_v1_routes;

mod v1;

const SWAGGER_URL: &str = "/swagger/openapi.yml";

pub fn create_routes() -> Router {
    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(cors::Any);
    let v1_routes = create_v1_routes();

    Router::new()
        .route("/", get(|| async { Redirect::permanent("/swagger") }))
        .route("/swagger", get(|| async { Html(swagger_ui(SWAGGER_URL)) }))
        .route(
            SWAGGER_URL,
            get(|| async { include_str!("../openapi.yml") }),
        )
        .nest("/", v1_routes.to_owned())
        .nest("/v1", v1_routes.to_owned())
        .nest("/api/v1", v1_routes)
        .layer(cors)
}
