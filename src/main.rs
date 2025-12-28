use axum::{extract::MatchedPath, http::Request};
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = tapciify_api::create_routes().layer(TraceLayer::new_for_http().make_span_with(
        |request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            tracing::info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        },
    ));

    let port = env::var("PORT").map_or(3000, |port| port.parse().unwrap());
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
