#![feature(file_create_new)]
mod config;
mod db;
mod model;

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

use crate::config::get_data_dir;

#[derive(Debug, Clone)]
struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // dbg!(get_database_file());

    let app_state = AppState {
        db: DB::connect(&get_database_file()?).await?,
    };

    let app = Router::new()
        .route("/*path", any(root))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    tracing::info!("Axum has ended");
    Ok(())
}

async fn handle_incoming_request(
    state: AppState,
    request: Request<Body>,
) -> anyhow::Result<impl IntoResponse> {
    let uri = request.uri();
    if uri.path() == "/hello" {
        let rows = state.db.get_recent_requests().await?;
        return Ok(Json(rows));
    }
    dbg!(uri);
    let body = request.body();
    let j = body.clone();
    let uuid = uuid::Uuid::new_v4();
    state
        .db
        .insert_request(&RequestData {
            uuid,
            uri: uri.clone(),
            headers: request.headers().clone(),
            method: request.method().clone(),
            body: vec![] // TODO
        })
        .await?;
    Ok(Json(vec![]))
}
// TODO:  Error handling -- Any T that implements From<T> for StatusCode should not able handled by INTERNAL SERVER ERROR

// basic handler that responds with a static string
// #[axum_macros::debug_handler]
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

// async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
//     (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         format!("Something went wrong: {}", err),
//     )
// }
