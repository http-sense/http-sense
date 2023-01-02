use std::sync::Arc;

use crate::models::Response;
use crate::models::Request;
use crate::models::ResponseSuccessData;
use anyhow::Context;

use serde::Serialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct DB {
    pool: SqlitePool,
}

#[derive(Clone, Debug, Serialize)]
pub struct DBRequest {
    request_id: i64,
    request_content: String,
    response_content: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ReqRes {
    pub request_id: i64,
    pub request: Request,
    pub response: Option<Response>,
}

impl ReqRes {
    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::json!({
            "request_id": self.request_id,
            "request": self.request.serialize_utf8_body(),
            "response": self.response.clone().map(|x| x.serialize_utf8_body()),
        })
    }
}

impl TryFrom<DBRequest> for ReqRes {
    type Error = anyhow::Error;
    fn try_from(value: DBRequest) -> anyhow::Result<Self> {
        Ok(ReqRes {
            request_id: value.request_id,
            request: serde_json::from_str(&value.request_content)?,
            response: Response::parse_json(&value.request_content),
        })
    }
}

impl TryFrom<DBRequest> for Request {
    type Error = anyhow::Error;
    fn try_from(value: DBRequest) -> anyhow::Result<Self> {
        let result: Self = serde_json::from_str(&value.request_content)?;
        Ok(result)
    }
}

impl TryFrom<DBRequest> for ResponseSuccessData {
    type Error = anyhow::Error;
    fn try_from(value: DBRequest) -> anyhow::Result<Self> {
        let result: Self = serde_json::from_str(
            &value
                .response_content
                .context("request does not have a response")?,
        )?;
        Ok(result)
    }
}

impl DB {
    pub async fn connect(db_file: &str) -> anyhow::Result<Self> {
        let db = Self {
            pool: SqlitePoolOptions::new()
                .max_connections(3)
                .connect(db_file)
                .await?,
        };
        db.bootstrap().await?;
        Ok(db)
    }

    async fn bootstrap(&self) -> anyhow::Result<()> {
        let query: &'static str = include_str!("../sqls/init.sql");
        sqlx::query(query).fetch_all(&self.pool).await?;

        Ok(())
    }

    pub async fn insert_request(&self, req: &Request) -> anyhow::Result<u64> {
        // Method
        let content = serde_json::to_string(req)?;

        // http_serde::header_map::serialize(&req.headers, ser)
        let _r = sqlx::query!("INSERT INTO request (content) VALUES (?)", content)
            .fetch_all(&self.pool)
            .await?;

        Ok(1)
    }

    pub async fn insert_response(&self, request_id: u64, response: &Response) -> anyhow::Result<()> {
        let content = serde_json::to_string(response)?;

        let req_id = request_id as i64;
        sqlx::query!(
            "INSERT INTO response (content, request_id) VALUES (?, ?)",
            content,
            req_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_recent_requests(&self) -> anyhow::Result<Vec<ReqRes>> {
        let result = sqlx::query_as!(
            DBRequest,
            "
            SELECT
                req.id as request_id,
                res.content as response_content,
                req.content as request_content
            FROM request req
            LEFT JOIN response res
            ON res.request_id = req.id;
        "
        )
        .fetch_all(&self.pool)
        .await?;
        return Ok(result.into_iter().map(|x| x.try_into()).collect::<anyhow::Result<Vec<_>>>()?);
    }
}

#[async_trait::async_trait]
pub trait RequestStorage {
    // Using `u64` here, but can be made generic T: Hash+Clone if a provider does not have u64 id
    async fn store_request(&mut self, req: &Request) -> anyhow::Result<u64>;
    async fn store_response(&mut self, request_id: u64, res: &Response) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl RequestStorage for DB {
    async fn store_request(&mut self, req: &Request) -> anyhow::Result<u64> {
        self.insert_request(req).await
    }
    async fn store_response(&mut self, request_id: u64, res: &Response) -> anyhow::Result<()> {
        self.insert_response(request_id, res).await
    }
}

#[async_trait::async_trait]
impl RequestStorage for Arc<DB> {
    async fn store_request(&mut self, req: &Request) -> anyhow::Result<u64> {
        self.insert_request(req).await
    }
    async fn store_response(&mut self, request_id: u64, res: &Response) -> anyhow::Result<()> {
        self.insert_response(request_id, res).await
    }
}

// TODO: revive
// #[cfg(test)]
// mod tests {
//     use http::Uri;

//     use super::*;
//     use std::env;

//     #[tokio::test]
//     async fn inserts_should_work() -> anyhow::Result<()> {
//         let tmpdir = tempfile::tempdir().unwrap();
//         env::set_current_dir(&tmpdir).unwrap();

//         let b = std::fs::File::create("test.sqlite");
//         drop(b);

//         let mut db = DB::connect("sqlite://test.sqlite").await?;
//         let results = db.get_recent_requests().await?;
//         assert!(results.is_empty());
//         db.insert_request(&RequestData {uri: "JustChecking".parse().unwrap()}).await?;

//         let results = db.get_recent_requests().await?;
//         assert_eq!(results.len(),  1);
//         assert_eq!(results[0].uri,  "JustChecking".parse::<Uri>().unwrap());

//         Ok(())
//     }

// }
