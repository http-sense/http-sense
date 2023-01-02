use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseSuccessData {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,

    #[serde(with = "http_serde::header_map")]
    pub headers: http::HeaderMap,

    pub body: bytes::Bytes,

    #[serde(with = "chrono::serde::ts_milliseconds", rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ResponseSuccessData {
    pub fn into_http_response(&self) -> anyhow::Result<http::Response<hyper::Body>> {
            let response_builder = http::Response::builder();
            let mut builder = response_builder.status(self.status_code);
            for (name, value) in self.headers.iter() {
                if name != hyper::header::TRANSFER_ENCODING {
                    builder = builder.header(name, value);
                }
            }
            let response_bytes = self.body.clone();

            let body = hyper::Body::from(response_bytes.clone());

            Ok(builder.body(body)?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseErrorData {
    #[serde(with = "chrono::serde::ts_milliseconds", rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type", rename_all="lowercase")]
pub enum Response {
    Success(ResponseSuccessData),
    Error(ResponseErrorData)
}

impl Response {

    pub fn serialize_map(&self) -> serde_json::Map<String, Value> {
        let mut v = json!(self);
        v.as_object_mut().cloned().unwrap()
    }

    /// Serialize the request adding body_size field
    /// and converting body to utf-8 if possible
    pub fn serialize_utf8_body(&self) -> Value {
        let mut rv = self.serialize_map();

        if let Self::Success(res) = self {
            // Replace the `body` field.
            rv.insert("body".to_string(), json!(res.body.to_utf8_string()));
            rv.insert("body_size".to_string(), json!(res.body.len()));
        }
        json!(rv)
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        match self {
            Self::Error(e) => e.created_at,
            Self::Success(e) => e.created_at,
        }
    }
    pub fn parse_json(value: &str) -> Option<Self> {
        if let Ok(x) = serde_json::from_str::<ResponseSuccessData>(&value) {
            Some(Self::Success(x))
        }
        else if let Ok(x) = serde_json::from_str::<ResponseErrorData>(&value) {
            Some(Self::Error(x))
        }
        else {
            None
        }
    }

}

impl From<ResponseErrorData> for Response {
    fn from(value: ResponseErrorData) -> Self {
        Self::Error(value)
    }
}

impl From<ResponseSuccessData> for Response {
    fn from(value: ResponseSuccessData) -> Self {
        Self::Success(value)
    }
}