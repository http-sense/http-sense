use crate::{
    models::{ResponseSuccessData, ResponseErrorData, Response, Request},
    axum_utils::*
};

use axum::{
    body::Body,
    extract::State,
    routing::any,
    Router,
};
use parking_lot::Mutex;



use std::{fmt::Debug, net::SocketAddr, sync::Arc, ops::AddAssign};


#[derive(Debug, Clone)]
pub enum RequestEvent<T> {
    Request(T, crate::models::Request),
    Response(T, Response),
}


impl<T: std::fmt::Display> RequestEvent<T> {
    pub fn serialize_response(&self) -> serde_json::Value {
        macro_rules! ser {
            ($i:ident, $d:ident) => {
                return {
                    let mut v: serde_json::Value = $d.serialize_utf8_body();
                    let obj = v.as_object_mut().unwrap();
                    obj.insert("id".to_string(), serde_json::Value::String($i.to_string()));
                    v
                }
            };
        }
        match self {
            Self::Request(i, d) => ser!(i, d),
            Self::Response(i, d) => ser!(i, d),
        }
    }
}

type RequestId = u64;

pub type ProxyEvent = RequestEvent<RequestId>;
#[derive(Debug)]
struct AppState {
    event_tx: tokio::sync::broadcast::Sender<ProxyEvent>,
    origin: url::Url,
    request_count: Mutex<usize>
}

pub async fn start_server(
    tx: tokio::sync::broadcast::Sender<ProxyEvent>,
    proxy_port: u16,
    proxy_addr: &str,
    origin: url::Url,
) -> anyhow::Result<()> {
    let app_state = Arc::new(AppState {
        event_tx: tx,
        origin: origin.clone(),
        request_count: Mutex::new(0)
    });

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

async fn make_proxy_request(request: crate::models::Request, origin: &url::Url) -> anyhow::Result<crate::models::ResponseSuccessData> {
    let uri = request.uri.clone();
    let headers = request.headers.clone();
    let method = request.method.clone();
    let body = request.body.clone();

    let method2 = method.clone();
    let make_request =
        async || -> anyhow::Result<crate::models::ResponseSuccessData> {
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
            let data = bd.body(body);
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

            let response_data = ResponseSuccessData {
                body: response_bytes,
                headers: response_headers,
                status_code: response_status,
                created_at: chrono::Utc::now(),
            };
            Ok(response_data)
        };

    make_request().await
}

#[axum_macros::debug_handler]
async fn root(
    State(state): State<Arc<AppState>>,
    request: http::Request<Body>,
) -> AxumResult<http::Response<hyper::Body>> {
    let req = crate::models::Request::from_http_request(request, chrono::Utc::now()).await?;

    let request_id = {
        let mut req_count = state.request_count.lock();
        *req_count += 1;
        let request_id = *req_count;
        drop(req_count); // release mutex
        request_id
    };

    state.event_tx.send(ProxyEvent::Request(request_id as u64, req.clone()))?;

    let origin = state.origin.clone();
    match make_proxy_request(req, &origin).await {
        Ok(response) => {
            state.event_tx.send(ProxyEvent::Response(request_id as u64, crate::models::Response::Success(response.clone())))?;
            // Convet hyper http:response from  models::Response
            Ok(response.into_http_response()?)
        }
        Err(e) => {
            state.event_tx.send(ProxyEvent::Response(
                request_id as u64,
                Response::Error(
                ResponseErrorData {
                    created_at: chrono::Utc::now(),
                    error: format!("{e}"),
                }),
            ))?;
            Err(e.into())
        }
    }
}


// #[axum_macros::debug_handler]
// async fn root(
//     State(state): State<Arc<AppState>>,
//     request: Request<Body>,
// ) -> AxumResult<http::Response<hyper::Body>> {
//     let uri = request.uri().clone();
//     let headers = request.headers().clone();
//     let method = request.method().clone();
//     let body = request.into_body();
//     let data = hyper::body::to_bytes(body).await?;
//     // TODO: see how threading is done in axum, and if it's single thread we can
//     // get away without using a Mutex
//     let mut req_count = state.request_count.lock();
//     *req_count += 1;
//     let request_id = *req_count;
//     drop(req_count); // release mutex
//     state.event_tx.send(
//         (
//             uuid,
//             Request {
//                 uri: uri.clone(),
//                 headers: headers.clone(),
//                 method: method.clone(),
//                 body: data.clone(), // TODO
//                 created_at: chrono::Utc::now(),
//             },
//         )
//             .into(),
//     )?;
//     let origin = state.origin.clone();
//     let method2 = method.clone();
//     let make_request =
//         async || -> anyhow::Result<(ResponseSuccessData, axum::response::Response<axum::body::Body>)> {
//             let client = reqwest::Client::builder()
//                 .redirect(reqwest::redirect::Policy::none())
//                 .gzip(true)
//                 .brotli(true)
//                 .deflate(true)
//                 .build()?;
//             let req_uri = &uri.to_string();
//             let value = origin.clone().join(&req_uri)?;
//             let mut bd = client.request(method2, value);
//             for (name, value) in headers.iter() {
//                 if name != hyper::header::TRANSFER_ENCODING && name != hyper::header::HOST {
//                     bd = bd.header(name, value);
//                 }
//             }
//             bd = bd.header(hyper::header::HOST, origin.host_str().unwrap());
//             let data = bd.body(data);
//             let response = data.send().await?;
//             let response_builder = http::Response::builder();
//             let mut builder = response_builder.status(response.status());
//             let response_headers = response.headers().clone();
//             let response_status = response.status();
//             for (name, value) in response.headers().iter() {
//                 if name != hyper::header::TRANSFER_ENCODING {
//                     builder = builder.header(name, value);
//                 }
//             }
//             let response_bytes = response.bytes().await?;

//             let body = hyper::Body::from(response_bytes.clone());

//             let res = builder.body(body)?;

//             let response_data = ResponseSuccessData {
//                 body: response_bytes,
//                 headers: response_headers,
//                 status_code: response_status,
//                 created_at: chrono::Utc::now(),
//             };
//             Ok((response_data, res))
//         };

//     match make_request().await {
//         Ok((response_data, res)) => {
//             state.event_tx.send((uuid, response_data).into())?;
//             Ok(res)
//         }
//         Err(e) => {
//             state.event_tx.send(ProxyEvent::RequestError(
//                 uuid,
//                 ResponseErrorData {
//                     created_at: chrono::Utc::now(),
//                     error: format!("{e}"),
//                 },
//             ))?;
//             Err(e.into())
//         }
//     }
// }
