use include_dir::{include_dir, Dir};
use crate::axum_utils::*;
use crate::db::{DB};

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

#[derive(Debug, Clone)]
struct AppState {
    db: Arc<DB>,
}
static PROJECT_DIR: Dir<'_> = include_dir!("$OUT_DIR/frontend_build");

// Endpoints
#[axum_macros::debug_handler]
async fn get_requests(
    State(state): State<AppState>,
    _request: Request<Body>,
) -> AxumResult<impl IntoResponse> {
    let rows = state.db.get_recent_requests().await?;
    let rows = rows.into_iter().map(|x| x).collect::<Vec<_>>();

    return Ok(Json(serde_json::json!(rows)));
}

#[axum_macros::debug_handler]
async fn get_frontend(
    State(_state): State<AppState>,
    request: Request<Body>,
) -> AxumResult<axum::response::Response> {
    let file_path = request.uri().to_string();
    let mut b = file_path.as_str();
    let b_orig = b;
    let b_html = format!("{}.html", b_orig);
    let b_dir_html = format!("{}/index.html", b_orig);

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

    match PROJECT_DIR.get_file(b) {
        Some(x) => {
            tracing::debug!("Returning file for {}", b);
            let body: axum::body::Full<Bytes> = x.contents().into();
            let response_mime: &'static str = mime_guess::from_path(b).first_raw().unwrap();

            Ok((
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


pub async fn start_server(db: Arc<DB>, ui_port: u16, ui_addr: &str) -> anyhow::Result<()> {
    let app_state = AppState { db };

    let app = Router::new()
        .route("/api/requests", get(get_requests))
        .route("/*path", get(get_frontend))
        .route("/", get(get_frontend))
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", ui_addr, ui_port).parse()?;
    let title = ansi_term::Style::new().bold();
    println!("   {} -> http://{}\n", title.paint("API Server"), addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    tracing::info!("UI Server has ended");
    Ok(())
}
