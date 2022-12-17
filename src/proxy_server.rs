use crate::{config::get_database_file, db::{DB, RequestStorage}, model::{RequestData, ResponseData}};
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
use std::{collections::HashMap, net::SocketAddr, sync::Arc};


#[derive(Debug, Clone)]
struct AppState<T: RequestStorage>{
    db: T,
    origin: url::Url
}

// impl AppState<T> {
//     async fn store_request(&mut self, req: &RequestData) -> anyhow::Result<Vec<u64>> {
//         let futures = self.db.iter_mut(|x| {
//             x.store_
//         })
//     }
// }

pub async fn start_server(db: Arc<DB>, proxy_port: u16, proxy_addr: &str, origin: &str) -> anyhow::Result<()> {
    let origin = url::Url::parse(origin)?;
    // let app_state = AppState { db: vec![db], origin};
    let app_state = AppState { db: db, origin};

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
    mut state: AppState<impl RequestStorage>,
    mut request: Request<Body>,
// ) -> anyhow::Result<impl IntoResponse> {
) -> anyhow::Result<http::Response<hyper::Body>> {
    let uri = request.uri().clone();
    let headers = request.headers().clone();
    let method =  request.method().clone();
    let mut body = request.into_body();
    let data = hyper::body::to_bytes(body).await?;
    let uuid = uuid::Uuid::new_v4();
    let request_id = state
        .db
        .store_request(&RequestData {
            uri,
            headers,
            method,
            body: data // TODO
        })
        .await?;
    
    let response = reqwest::get(state.origin).await?;
    let response_builder = http::Response::builder();
    let mut builder = response_builder.status(response.status());
    let response_headers = response.headers().clone();
    let response_status = response.status();
    for (name, value) in response.headers().iter() {
        builder = builder.header(name, value);
    }
    let response_bytes = response.bytes().await?;
    let body = hyper::Body::from(response_bytes.clone());
    let res = builder.body(body)?;

    let response_data = ResponseData {
        body: response_bytes,
        headers: response_headers,
        status_code: response_status,
        request_id,
    };
    state.db.store_response(&response_data).await?;

    return Ok(res);

    // return Ok(response);
    
    // Ok(Json("Trust me, this is the response your server gave!"))

}
// TODO:  Error handling -- Any T that implements From<T> for StatusCode should not able handled by INTERNAL SERVER ERROR

// basic handler that responds with a static string
// #[axum_macros::debug_handler]
async fn root(
    State(state): State<AppState<impl RequestStorage>>,
    request: Request<Body>,
) -> Result<Response<hyper::Body>, StatusCode> {
    handle_incoming_request(state, request)
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}