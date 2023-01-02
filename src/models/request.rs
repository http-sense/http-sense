use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    #[serde(with = "http_serde::uri")]
    pub uri: http::Uri,

    #[serde(with = "http_serde::method")]
    pub method: http::Method,

    #[serde(with = "http_serde::header_map")]
    pub headers: http::HeaderMap,

    pub body: bytes::Bytes,

    #[serde(with = "chrono::serde::ts_milliseconds", rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Request {

    pub fn serialize_map(&self) -> serde_json::Map<String, Value> {
        let mut v = json!(self);
        v.as_object_mut().cloned().unwrap()
    }

    /// Serialize the request adding body_size field
    /// and converting body to utf-8 if possible
    pub fn serialize_utf8_body(&self) -> Value {
        let mut rv = self.serialize_map();

        // Replace the `body` field.
        rv.insert("body".to_string(), json!(self.body.to_utf8_string()));
        rv.insert("body_size".to_string(), json!(self.body.len()));
        json!(rv)
    }

    pub async fn from_http_request(request: http::Request<hyper::Body>, created_at: chrono::DateTime<chrono::Utc>) -> anyhow::Result<Self> {
        let uri = request.uri().clone();
        let headers = request.headers().clone();
        let method = request.method().clone();
        let body = request.into_body();
        let data = hyper::body::to_bytes(body).await?;
        Ok(Request {
            uri,
            method,
            body: data,
            headers,
            created_at
        })
    }
}

// Problem no. 1 => 