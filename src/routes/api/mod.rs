pub mod v1;

use axum::Router;
use v1::create_v1_routes;

pub fn create_api_routes() -> Router {
    let v1 = create_v1_routes();

    Router::new().nest("/v1", v1)
}
