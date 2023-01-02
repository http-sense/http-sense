use include_dir::{include_dir, Dir};
use axum::extract::ws::{
    Message, WebSocket, WebSocketUpgrade
};
use crate::axum_utils::*;
use crate::db::{DB, ReqRes};
use crate::proxy_server::RequestEvent;
use tower_http::cors::CorsLayer;

use axum::{
    body::{Body, Bytes},
    extract::State,
    http::Request,
    http::StatusCode,
    response::{IntoResponse},
    routing::get,
    Json, Router,
};

use std::{net::SocketAddr, sync::Arc};

#[derive(Debug)]
struct AppState {
    db: Arc<DB>,
    event_rx: tokio::sync::broadcast::Receiver<RequestEvent<u64>>
}
static PROJECT_DIR: Dir<'_> = include_dir!("$OUT_DIR/frontend_build");

// Endpoints
#[axum_macros::debug_handler]
async fn get_requests(
    State(state): State<Arc<AppState>>,
    _request: Request<Body>,
) -> AxumResult<impl IntoResponse> {
    let rows = state.db.get_recent_requests().await?;
    let rows = rows.into_iter().map(|x| x.to_json_value()).collect::<Vec<_>>();
    return Ok(Json(serde_json::json!(rows)));
}

#[axum_macros::debug_handler]
async fn get_frontend(
    State(_state): State<Arc<AppState>>,
    request: Request<Body>,
) -> AxumResult<axum::response::Response> {
    let file_path = request.uri().path().to_string();
    let mut b = file_path.as_str();
    let b_orig = b;
    let b_html = format!("{}.html", b_orig);
    let b_dir_html = format!("{}/index.html", b_orig);
    let mut status = StatusCode::OK;

    while b.starts_with("/") {
        b = &b[1..];
    }

    if PROJECT_DIR.get_file(b).is_none() {
        b = b_html.as_str();
    }
    while b.starts_with("/") {
        b = &b[1..];
    }
    if PROJECT_DIR.get_file(b).is_none() {
        b = b_dir_html.as_str();
    }
    while b.starts_with("/") {
        b = &b[1..];
    }
    if PROJECT_DIR.get_file(b).is_none() {
        status = StatusCode::NOT_FOUND;
        b = "index.html";
    }

    match PROJECT_DIR.get_file(b) {
        Some(x) => {
            tracing::debug!("Returning file for {}", b);
            let body: axum::body::Full<Bytes> = x.contents().into();
            let response_mime: &'static str = mime_guess::from_path(b).first_raw().unwrap();

            Ok((
                status,
                [(
                    http::header::CONTENT_TYPE,
                    http::HeaderValue::from_static(response_mime),
                )],
                body,
            )
                .into_response())
        }
        None => Ok((StatusCode::NOT_FOUND, "Hey, get out of here").into_response()),
    }
}

#[axum_macros::debug_handler]
async fn set_websocket(
    ws: WebSocketUpgrade,
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    let mut rx = _state.event_rx.resubscribe();
    ws.on_upgrade(async move |mut socket| {
        loop {
            let val = match rx.recv().await {
                Ok(v) => v,
                Err(e) => {
                    println!("Event Reciever dead. Closing websocket: {:?}", e);
                    return
                }
            };
            let res = val.serialize_response();
            if let Err(e) = socket.send(Message::Text(serde_json::to_string(&res).unwrap())).await {
                println!("Websocket client disconnect: {:?}. Closing", e);
                return 
            };
        }
    })
}


pub async fn start_server(db: Arc<DB>, ui_port: u16, ui_addr: &str, event_rx: tokio::sync::broadcast::Receiver<RequestEvent<u64>>) -> anyhow::Result<()> {
    let app_state = Arc::new(AppState { db, event_rx });

    let app = Router::new()
        .route("/api/requests", get(get_requests))
        .route("/api/ws", get(set_websocket))
        .route("/*path", get(get_frontend))
        .route("/", get(get_frontend))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", ui_addr, ui_port).parse()?;
    let title = ansi_term::Style::new().bold();
    println!("   {} -> http://{}\n", title.paint("API Server"), addr);
    if cfg!(debug_assertions) {
        // Easier to build local frontend
        println!("   {} -> http://{}\n", title.paint("API Svelte Server"), format!("localhost:5173/?api_url=http://localhost:{}/api", ui_port));
    }

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    tracing::info!("UI Server has ended");
    Ok(())
}
