use crate::{config::get_database_file, db::DB, model::RequestData};
use anyhow::Context;
use axum::{
    body::{Body, Bytes},
    extract::{Query, State},
    http::StatusCode,
    http::{header::HeaderMap, Request},
    response::IntoResponse,
    routing::{any, get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};


#[derive(Debug, Clone)]
struct AppState {
    db: Arc<DB>,
}

pub async fn start_server(db: Arc<DB>, ui_port: u16, ui_addr: &str) -> anyhow::Result<()> {
    let app_state = AppState { db };

    let app = Router::new()
        .route("/*path", get(root))
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", ui_addr, ui_port).parse()?;
    tracing::info!("ui server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    tracing::info!("UI Server has ended");
    Ok(())

}

async fn handle_incoming_request(
    state: AppState,
    request: Request<Body>,
) -> anyhow::Result<impl IntoResponse> {
    let rows = state.db.get_recent_requests().await?;
    return Ok(Json(rows));

}
// TODO:  Error handling -- Any T that implements From<T> for StatusCode should not be handled by INTERNAL SERVER ERROR

// basic handler that responds with a static string
#[axum_macros::debug_handler]
async fn root(
    State(state): State<AppState>,
    request: Request<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    handle_incoming_request(state, request)
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}