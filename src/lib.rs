use axum::{
    http::Method,
    response::{Html, Redirect},
    routing::get,
    Router,
};
use swagger::generate_swagger;
use tower_http::cors;
use v1::create_v1_routes;

mod swagger;
mod v1;

const V1_SWAGGER_URL: &str = "/v1/swagger/openapi.yml";

pub fn create_docs_routes() -> Router {
    let v1_swagget_ui = generate_swagger(V1_SWAGGER_URL);

    Router::new()
        .route("/", get(|| async { Redirect::permanent("/v1") }))
        .route(
            "/swagger",
            get(|| async { Redirect::permanent("/v1/swagger") }),
        )
        .route("/v1", get(|| async { Redirect::permanent("/v1/swagger") }))
        .route("/v1/swagger", get(|| async { Html(v1_swagget_ui) }))
        .route(
            V1_SWAGGER_URL,
            get(|| async { include_str!("./v1/openapi.yml") }),
        )
}

pub fn create_routes() -> Router {
    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(cors::Any);

    let docs_routes = create_docs_routes();
    let v1_routes = create_v1_routes();

    Router::new()
        .nest("/", v1_routes.to_owned())
        .nest("/v1", v1_routes.to_owned())
        .nest("/api/v1", v1_routes)
        .merge(docs_routes)
        .layer(cors)
}
