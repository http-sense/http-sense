#![feature(file_create_new)]
mod db;
mod config;
mod model;

use crate::{db::DB, config::get_database_file, model::RequestData};
use anyhow::Context;
use axum::{
    routing::{get, post, any},
    http::StatusCode,
    http::{Request, header::HeaderMap },
    response::IntoResponse,
        body::{Bytes, Body},
    Json, Router, extract::{Query, State},
};

use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, collections::HashMap, sync::Arc};

use crate::config::get_data_dir;

#[derive(Debug, Clone)]
struct AppState {
    db: DB
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dbg!(get_database_file());

    let mut app_state = AppState {
        db: DB::connect(&get_database_file()?).await?
    };

    let app = Router::new()
        .route("/*path", any(root))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}


async fn handle_incoming_request(state: AppState, request: Request<Body>) -> anyhow::Result<&'static str> {
    let uri = request.uri();
    dbg!(uri);
    state.db.insert_request(&RequestData {
        uri: uri.clone()
    }).await?;
    Ok("Hello World")

}
// TODO:  Error handling -- Any T that implements From<T> for StatusCode should not able handled by INTERNAL SERVER ERROR

// basic handler that responds with a static string
#[axum_macros::debug_handler]
async fn root(State(state): State<AppState>, request: Request<Body>) -> Result<&'static str, StatusCode> {
    handle_incoming_request(state, request).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("Hello, World!")
}

async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}