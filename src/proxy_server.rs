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
use std::{collections::HashMap, net::SocketAddr, sync::Arc, fmt::Debug};

trait MyTrait: RequestStorage + std::fmt::Debug {}

#[derive(Debug)]
struct AppState {
    // db: [Box<dyn MyTrait>; 5],
    db: (Box<dyn MyTrait>, Box<dyn MyTrait>, Box<dyn MyTrait>, Box<dyn MyTrait>, Box<dyn MyTrait>),
    origin: url::Url
}


struct ResponseWriter<'a> {
    app_state: &'a mut AppState,
    ids: Vec<u64>
}
#[derive(Debug)]
struct NoOpWriter { }
#[async_trait::async_trait]
impl RequestStorage for NoOpWriter {
    async fn store_request(&mut self, req: &RequestData) -> anyhow::Result<u64> {
        Ok(0)
    }
    async fn store_response(&mut self, req: &ResponseData) -> anyhow::Result<()> {
        Ok(())
    }
}

// impl MyTrait for NoOpWriter {}

impl<T: Debug + RequestStorage> MyTrait for T { }
impl AppState {

    fn new(mut dbs: Vec<Box<dyn MyTrait>>, origin: url::Url) -> Self {
        assert!(dbs.len() <= 5, "Max 5 request storages support atm");
        while (dbs.len() < 5) {
            dbs.push(Box::new(NoOpWriter{}));
        }

        AppState { db: (dbs[0], dbs[1], dbs[2], dbs[3], dbs[4]), origin }
    }

    async fn write_request<'a>(&'a mut self, req: &RequestData) -> anyhow::Result<ResponseWriter<'a>> {
        let res = futures::future::join5(
            self.db.0.store_request(req),
            self.db.1.store_request(req),
            self.db.2.store_request(req),
            self.db.3.store_request(req),
            self.db.4.store_request(req),
        ).await;
        let res = vec![
            res.0?,
            res.1?,
            res.2?,
            res.3?,
            res.4?,
        ];
        Ok(ResponseWriter {
            app_state: self,
            ids: res

        })
        // let rs = futures::future::join_all(tasks).await;
        // let b: anyhow::Result<Vec<u64>> = rs.into_iter().map(|x| {
        //     x?
        // }).collect();
        // let b = b?;

        // todo!()
        // a.store_request(req).await;
    }
}


pub async fn start_server(db: Arc<DB>, proxy_port: u16, proxy_addr: &str, origin: &str) -> anyhow::Result<()> {
    let origin = url::Url::parse(origin)?;
    // let app_state = AppState { db: vec![db], origin};
    let app_state = Arc::new(AppState::new(vec![Box::new(db)], origin));

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