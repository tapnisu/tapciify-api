pub mod query;
pub mod routes;

use routes::create_routes;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_routes();

    let addr = SocketAddr::from((
        [127, 0, 0, 1],
        env::var("PORT")
            .map(|port| port.parse().unwrap())
            .unwrap_or(3000),
    ));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
