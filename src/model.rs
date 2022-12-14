use std::{collections::HashMap, str::Bytes};

use serde::{Deserialize, Serialize};

// path + query params = uri
// headers = hashmap
// body = bytes
// body = bytes


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestData {
    // This identifier is same for the reponse_data
    pub uuid: uuid::Uuid,

    #[serde(with = "http_serde::uri")]
    pub uri: http::Uri,

    #[serde(with = "http_serde::method")]
    pub method: http::Method,

    #[serde(with = "http_serde::header_map")]
    pub headers: http::HeaderMap,

    pub body: bytes::Bytes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseData {
    pub uuid: uuid::Uuid,

    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,

    #[serde(with = "http_serde::header_map")]
    pub headers: http::HeaderMap,

    pub body: bytes::Bytes,
}