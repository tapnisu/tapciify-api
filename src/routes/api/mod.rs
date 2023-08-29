pub mod v1;
mod v2;

use axum::Router;
use v1::create_v1_routes;
use v2::create_v2_routes;

pub fn create_api_routes() -> Router {
    let v1 = create_v1_routes();
    let v2 = create_v2_routes();

    Router::new()
        .nest("/", v2.clone())
        .nest("/v1", v1)
        .nest("/v2", v2)
}
