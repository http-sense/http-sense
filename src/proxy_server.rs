use crate::{config::get_database_file, db::{DB, RequestStorage}, model::{RequestData, ResponseData, ResponseError}};
use anyhow::Context;
use axum::{
    body::{Body, Bytes},
    extract::{Query, State},
    http::StatusCode,
    http::{header::HeaderMap, Request},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, fmt::Debug};

trait MyTrait: RequestStorage + std::fmt::Debug {}
type RequestId = uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ProxyEvent {
    // TODO: move RequestId out
    RequestRecv(RequestId, RequestData),
    ResponseRecv(RequestId, ResponseData),
    RequestError(RequestId, ResponseError)
}

impl From<(RequestId, RequestData)> for ProxyEvent {
    fn from(value: (RequestId, RequestData)) -> Self {
        ProxyEvent::RequestRecv(value.0, value.1)
    }
}

impl From<(RequestId, ResponseData)> for ProxyEvent {
    fn from(value: (RequestId, ResponseData)) -> Self {
        ProxyEvent::ResponseRecv(value.0, value.1)
    }
}

#[derive(Debug, Clone)]
struct AppState {
    // db: [Box<dyn MyTrait>; 5],
    event_tx: tokio::sync::broadcast::Sender<ProxyEvent>,
    origin: url::Url
}

impl<T: Debug + RequestStorage> MyTrait for T { }


pub async fn start_server(tx: tokio::sync::broadcast::Sender<ProxyEvent>, proxy_port: u16, proxy_addr: &str, origin: &str) -> anyhow::Result<()> {
    let origin = url::Url::parse(origin)?;
    // let (tx, mut rx) = tokio::sync::broadcast::channel(128);
    let app_state = AppState {
        event_tx: tx,
        origin,
    };

    let app = Router::new()
        .route("/*path", any(root))
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", proxy_addr, proxy_port).parse()?;
    tracing::info!("proxy server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    tracing::info!("Proxy Server has ended");
    Ok(())

}

async fn handle_incoming_request(
    mut state: AppState,
    mut request: Request<Body>,
// ) -> anyhow::Result<impl IntoResponse> {
) -> anyhow::Result<http::Response<hyper::Body>> {
    let uri = request.uri().clone();
    let headers = request.headers().clone();
    let method =  request.method().clone();
    let mut body = request.into_body();
    let data = hyper::body::to_bytes(body).await?;
    let uuid = uuid::Uuid::new_v4();
    state
        .event_tx.send((uuid, RequestData {
            uri,
            headers,
            method,
            body: data, // TODO
            createdAt: chrono::Utc::now()
        }).into())?;
    
    let make_request = async || -> anyhow::Result<(ResponseData, axum::response::Response<axum::body::Body>)> {
        let response = reqwest::get(state.origin).await?;
        let response_builder = http::Response::builder();
        let mut builder = response_builder.status(response.status());
        let response_headers = response.headers().clone();
        let response_status = response.status();
        for (name, value) in response.headers().iter() {
            dbg!(name, value);
            builder = builder.header(name, value);
        }
        let response_bytes = response.bytes().await?;
        let body = hyper::Body::from(response_bytes.clone());
        let res = builder.body(body)?;
        dbg!("Added body");

        let response_data = ResponseData {
            body: response_bytes,
            headers: response_headers,
            status_code: response_status,
            createdAt: chrono::Utc::now()
        };
        Ok((response_data, res))
    };

    match make_request().await {
        Ok((response_data, res)) => {
            state.event_tx.send((uuid, response_data).into())?;
            dbg!(&res);
            Ok(res)
        },
        Err(e) => {
            state.event_tx.send(ProxyEvent::RequestError(uuid, ResponseError { createdAt: chrono::Utc::now(), error: format!("{e}") }))?;
            Err(e)
        }
    }
}
// TODO:  Error handling -- Any T that implements From<T> for StatusCode should not able handled by INTERNAL SERVER ERROR

// basic handler that responds with a static string
#[axum_macros::debug_handler]
async fn root(
    State(state): State<AppState>,
    request: Request<Body>,
) -> Result<Response<hyper::Body>, StatusCode> {
    handle_incoming_request(state, request)
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
