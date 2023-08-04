pub mod routes;
pub mod structs;

use routes::create_routes;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let app = create_routes();

    Ok(app.into())
}
