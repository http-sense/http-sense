use std::{borrow::Borrow};
use serde_json:: {json};

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

impl RequestData {
    pub fn utf8_body(&self) -> anyhow::Result<String> {
        let x_body = self.body.clone();
        Ok(std::str::from_utf8(x_body.borrow()).map(|x| x.to_string())?)
    }

    pub fn body_size(&self) -> usize {
        self.body.len()
    }
    pub fn serialize_without_body(&self) -> serde_json::Value {
        let mut request_value = json!(self);
        let obj = request_value.as_object_mut().unwrap();
        obj.remove("body").unwrap();
        request_value
    }
    pub fn serialize_response(&self) -> serde_json::Value {
        let mut request_value = self.serialize_without_body();
        let content = request_value.as_object_mut().unwrap();
        content.insert("body".to_string(), json!(self.utf8_body().ok()));
        content.insert("body_size".to_string(), json!(self.body_size()));
        request_value
    }
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

impl ResponseData {
    pub fn utf8_body(&self) -> anyhow::Result<String> {
        let x_body = self.body.clone();
        Ok(std::str::from_utf8(x_body.borrow()).map(|x| x.to_string())?)
    }

    pub fn body_size(&self) -> usize {
        self.body.len()
    }

    pub fn serialize_without_body(&self) -> serde_json::Value {
        let mut request_value = json!(self);
        let obj = request_value.as_object_mut().unwrap();
        obj.remove("body").unwrap();
        request_value
    }

    pub fn serialize_response(&self) -> serde_json::Value {
        let mut request_value = self.serialize_without_body();
        let content = request_value.as_object_mut().unwrap();
        content.insert("body".to_string(), json!(self.utf8_body().ok()));
        content.insert("body_size".to_string(), json!(self.body_size()));
        request_value
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseError {
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub error: String
}
impl ResponseError {
    pub fn serialize_response(&self) -> serde_json::Value {
        json!(self)
    }
}