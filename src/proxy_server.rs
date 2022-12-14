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

pub async fn start_proxy(db: Arc<DB>, proxy_port: u16, proxy_addr: &str) -> anyhow::Result<()> {
    let app_state = AppState { db };

    let app = Router::new()
        .route("/*path", any(root))
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", proxy_addr, proxy_port).parse()?;
    tracing::info!("proxy server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    tracing::info!("Axum has ended");
    Ok(())

}

async fn handle_incoming_request(
    state: AppState,
    mut request: Request<Body>,
) -> anyhow::Result<impl IntoResponse> {
    let uri = request.uri().clone();
    if uri.path() == "/hello" {
        let rows = state.db.get_recent_requests().await?;
        return Ok(Json(rows));
    }
    let headers = request.headers().clone();
    let method =  request.method().clone();
    let mut body = request.into_body();
    // let j = body.clone();
    let data = hyper::body::to_bytes(body).await?;
    // let j = body.to_bytes();
    let uuid = uuid::Uuid::new_v4();
    state
        .db
        .insert_request(&RequestData {
            uuid,
            uri,
            headers,
            method,
            body: data // TODO
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