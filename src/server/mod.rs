use std::net::{IpAddr, SocketAddr};

use axum::{routing::get, Router};

pub(super) async fn launch(address: IpAddr, port: u16) {
    let app: Router = Router::new()
        .route("/", get(|| async { include_str!("../../assets/up.html") }))
        .fallback(|| async { include_str!("../../assets/404.html") });

    axum::Server::bind(&SocketAddr::new(address, port))
        .serve(app.into_make_service())
        .await;
}
