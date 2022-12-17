use std::{collections::HashMap, str::Bytes};

use serde::{Deserialize, Serialize};

// path + query params = uri
// headers = hashmap
// body = bytes
// body = bytes


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestData {
    #[serde(with = "http_serde::uri")]
    pub uri: http::Uri,

    #[serde(with = "http_serde::method")]
    pub method: http::Method,

    #[serde(with = "http_serde::header_map")]
    pub headers: http::HeaderMap,

    pub body: bytes::Bytes,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub createdAt: chrono::DateTime<chrono::Utc>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseData {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,

    #[serde(with = "http_serde::header_map")]
    pub headers: http::HeaderMap,

    pub body: bytes::Bytes,


    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub createdAt: chrono::DateTime<chrono::Utc>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseError {
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub createdAt: chrono::DateTime<chrono::Utc>
}