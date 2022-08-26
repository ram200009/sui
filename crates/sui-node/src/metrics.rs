// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use axum::{extract::Extension, http::StatusCode, routing::get, Router};
use prometheus::{Registry, TextEncoder};
use std::net::SocketAddr;

use tracing::warn;

const METRICS_ROUTE: &str = "/metrics";

pub fn start_prometheus_server(addr: SocketAddr) -> Registry {
    let registry = Registry::new();

    if cfg!(madsim) {
        // prometheus uses difficult-to-support features such as TcpSocket::from_raw_fd(), so we
        // can't yet run it in the simulator.
        warn!("not starting prometheus server in simulator");
        return registry;
    }

    let app = Router::new()
        .route(METRICS_ROUTE, get(metrics))
        .layer(Extension(registry.clone()));

    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    registry
}

async fn metrics(Extension(registry): Extension<Registry>) -> (StatusCode, String) {
    let metrics_families = registry.gather();
    match TextEncoder.encode_to_string(&metrics_families) {
        Ok(metrics) => (StatusCode::OK, metrics),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unable to encode metrics: {error}"),
        ),
    }
}
