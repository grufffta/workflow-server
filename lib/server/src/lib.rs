use axum::{response::Html, routing::get, Router};
use std::net::{IpAddr, SocketAddr};
#[allow(unused_imports)]
use tokio::signal::{self, unix::signal};

pub mod config;

pub async fn launch(address: IpAddr, port: u16) {
    let app: Router = Router::new()
        .route(
            "/",
            get(|| async { Html(include_str!("../assets/up.html")) }),
        )
        .fallback(|| async { Html(include_str!("../assets/404.html")) });

    axum::Server::bind(&SocketAddr::new(address, port))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_hook())
        .await
        .unwrap();
}

fn shutdown() {
    println!("\nServer Shutdown!")
}

async fn shutdown_hook() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install shutdown signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    shutdown();
}
