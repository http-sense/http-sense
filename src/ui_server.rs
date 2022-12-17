use include_dir::{include_dir, Dir};
use std::path::Path;

use crate::{config::get_database_file, db::DB, model::{RequestData, ResponseData}};
use anyhow::Context;
use axum::{
    body::{Body, Bytes},
    extract::{Query, State},
    http::StatusCode,
    http::{header::HeaderMap, Request},
    response::{IntoResponse, Html},
    routing::{any, get, post},
    Json, Router
};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};


#[derive(Debug, Clone)]
struct AppState {
    db: Arc<DB>,
}
static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/frontend/build");

pub async fn start_server(db: Arc<DB>, ui_port: u16, ui_addr: &str) -> anyhow::Result<()> {
    let app_state = AppState { db };

    let app = Router::new()
        .route("/api/requests", get(get_requests))
        .route("/api/responses", get(get_responses))
        // .route("/*path", get(get_frontend))
        .route("/*path", get(get_frontend))
        .route("/", get(get_frontend))
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
async fn get_requests(
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

async fn handle_incoming_req2(
    state: AppState,
    request: Request<Body>,
) -> anyhow::Result<impl IntoResponse> {
    let rows = state.db.get_recent_responses().await?;
    return Ok(Json(rows));

}

#[axum_macros::debug_handler]
async fn get_responses(
    State(state): State<AppState>,
    request: Request<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    handle_incoming_req2(state, request)
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}


async fn _get_frontend(
    state: AppState,
    request: Request<Body>,
) -> anyhow::Result<axum::response::Response> {
    let file_path = request.uri().to_string();
    let mut b = file_path.as_str();
    let b_orig = b;
    let b_html = format!("{}.html", b_orig);
    let b_dir_html = format!("{}/index.html", b_orig);

    while (b.starts_with("/")) {
        b = &b[1..];
    }

    dbg!(b);
    if (PROJECT_DIR.get_file(b).is_none()) {
        b = b_html.as_str();
        dbg!(b);
    }
    while (b.starts_with("/")) {
        b = &b[1..];
    }
    if (PROJECT_DIR.get_file(b).is_none()) {
        b = b_dir_html.as_str();
        dbg!(b);
    }
    while (b.starts_with("/")) {
        b = &b[1..];
    }

    match PROJECT_DIR.get_file(b) {
        Some(x) => {
            tracing::debug!("Returning file for {}", b);
            let body: axum::body::Full<Bytes> = x.contents().into();
            let response_mime: &'static str = mime_guess::from_path(b).first_raw().unwrap();

            Ok(([(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static(response_mime)
            )],
                body
            ).into_response())
        },
        None => {
            Ok((StatusCode::NOT_FOUND, "Hey, get out of here").into_response())
        }
    }
}

#[axum_macros::debug_handler]
async fn get_frontend(
    State(state): State<AppState>,
    request: Request<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    _get_frontend(state, request)
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}