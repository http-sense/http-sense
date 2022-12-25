use crate::{
    model::{RequestData, ResponseData, ResponseError},
    axum_utils::*
};

use axum::{
    body::Body,
    extract::{State},
    http::Request,
    routing::any,
    Router,
};



use std::{fmt::Debug, net::SocketAddr};

type RequestId = uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ProxyEvent {
    // TODO: move RequestId out
    RequestRecv(RequestId, RequestData),
    ResponseRecv(RequestId, ResponseData),
    RequestError(RequestId, ResponseError),
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
    event_tx: tokio::sync::broadcast::Sender<ProxyEvent>,
    origin: url::Url,
}

pub async fn start_server(
    tx: tokio::sync::broadcast::Sender<ProxyEvent>,
    proxy_port: u16,
    proxy_addr: &str,
    origin: url::Url,
) -> anyhow::Result<()> {
    let app_state = AppState {
        event_tx: tx,
        origin: origin.clone(),
    };

    let app = Router::new()
        .route("/", any(root))
        .route("/*path", any(root))
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", proxy_addr, proxy_port).parse()?;
    let title = ansi_term::Style::new().bold();
    println!("   {} -> http://{}", title.paint("Proxy Server"), addr);
    println!(
        "        {} -> {}\n",
        title.paint("Proxying to"),
        origin.to_string()
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    tracing::info!("Proxy Server has ended");
    Ok(())
}


#[axum_macros::debug_handler]
async fn root(
    State(state): State<AppState>,
    request: Request<Body>,
) -> AxumResult<http::Response<hyper::Body>> {
    let uri = request.uri().clone();
    let headers = request.headers().clone();
    let method = request.method().clone();
    let body = request.into_body();
    let data = hyper::body::to_bytes(body).await?;
    let uuid = uuid::Uuid::new_v4();
    state.event_tx.send(
        (
            uuid,
            RequestData {
                uri: uri.clone(),
                headers: headers.clone(),
                method: method.clone(),
                body: data.clone(), // TODO
                createdAt: chrono::Utc::now(),
            },
        )
            .into(),
    )?;

    let origin = state.origin.clone();
    let method2 = method.clone();
    let make_request =
        async || -> anyhow::Result<(ResponseData, axum::response::Response<axum::body::Body>)> {
            let client = reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .gzip(true)
                .brotli(true)
                .deflate(true)
                .build()?;
            let req_uri = &uri.to_string();
            let value = origin.clone().join(&req_uri)?;
            let mut bd = client.request(method2, value);

            for (name, value) in headers.iter() {
                if name != hyper::header::TRANSFER_ENCODING && name != hyper::header::HOST {
                    bd = bd.header(name, value);
                }
            }
            bd = bd.header(hyper::header::HOST, origin.host_str().unwrap());
            let data = bd.body(data);
            let response = data.send().await?;

            let response_builder = http::Response::builder();
            let mut builder = response_builder.status(response.status());
            let response_headers = response.headers().clone();
            let response_status = response.status();
            for (name, value) in response.headers().iter() {
                if name != hyper::header::TRANSFER_ENCODING {
                    builder = builder.header(name, value);
                }
            }
            let response_bytes = response.bytes().await?;

            let body = hyper::Body::from(response_bytes.clone());

            let res = builder.body(body)?;

            let response_data = ResponseData {
                body: response_bytes,
                headers: response_headers,
                status_code: response_status,
                createdAt: chrono::Utc::now(),
            };
            Ok((response_data, res))
        };

    match make_request().await {
        Ok((response_data, res)) => {
            state.event_tx.send((uuid, response_data).into())?;
            Ok(res)
        }
        Err(e) => {
            state.event_tx.send(ProxyEvent::RequestError(
                uuid,
                ResponseError {
                    createdAt: chrono::Utc::now(),
                    error: format!("{e}"),
                },
            ))?;
            Err(e.into())
        }
    }
}