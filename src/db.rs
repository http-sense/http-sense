use std::str::FromStr;
use std::sync::Arc;

use crate::model::RequestData;
use crate::model::ResponseData;
use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use sqlx::sqlite::SqliteConnection;
use sqlx::sqlite::SqliteExecutor;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Connection;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct DB {
    pool: SqlitePool,
}

#[derive(Clone, Debug)]
struct DBRequest {
    request_id: i64,

    request_ts: String,
    request_content: String,

    response_ts: Option<String>,
    response_content: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ReqRes {
    pub request_id: i64,

    pub request: (chrono::DateTime<chrono::Utc>, RequestData),

    pub response: Option<(chrono::DateTime<chrono::Utc>, ResponseData)>,
}


fn str_to_chrono(value: &str) -> anyhow::Result<chrono::DateTime<chrono::Utc>> {
    // 2
    let value = format!("{}+00:00", value);
    chrono::DateTime::from_str(&value).ok().context("Can't parse chrono date")
}

impl TryFrom<DBRequest> for ReqRes {
    type Error = anyhow::Error;
    fn try_from(value: DBRequest) -> anyhow::Result<Self> {
        let request_time = str_to_chrono(&value.request_ts)?;
        let mut response: Option<(chrono::DateTime<chrono::Utc>, ResponseData)> = None;
        if let Some(res_ts) = value.response_ts {
            response = Some((
                str_to_chrono(&res_ts)?,
                serde_json::from_str(
                    &value
                        .response_content
                        .context("response body is invalid or missing")?,
                )?,
            ))
        }
        Ok(ReqRes {
            request_id: value.request_id,
            request: (request_time, serde_json::from_str(&value.request_content)?),
            response,
        })
    }
}

impl TryFrom<DBRequest> for RequestData {
    type Error = anyhow::Error;
    fn try_from(value: DBRequest) -> anyhow::Result<Self> {
        let result: Self = serde_json::from_str(&value.request_content)?;
        Ok(result)
    }
}

impl TryFrom<DBRequest> for ResponseData {
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

    pub async fn insert_request(&self, req: &RequestData) -> anyhow::Result<u64> {
        // Method
        let content = serde_json::to_string(req)?;
        dbg!(&content);

        // http_serde::header_map::serialize(&req.headers, ser)
        let r = sqlx::query!("INSERT INTO request (content) VALUES (?)", content)
            .fetch_all(&self.pool)
            .await?;

        Ok(1)
    }

    pub async fn insert_response(&self, res: &ResponseData) -> anyhow::Result<()> {
        // Method
        let content = serde_json::to_string(res)?;

        let req_id = res.request_id as i64;
        // http_serde::header_map::serialize(&req.headers, ser)
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
        // let result = sqlx::query_as!(DBRow, "SELECT uuid, content FROM request")
        // // let result = sqlx::query_as!(DBRow, "SELECT content FROM request")
        //     .fetch_all(&self.pool)
        //     .await?;

        let result = sqlx::query_as!(
            DBRequest,
            "
            SELECT
                req.id as request_id,
                res.content as response_content,
                res.created_at as response_ts,
                req.content as request_content,
                req.created_at as request_ts
            FROM request req
            LEFT JOIN response res
            ON res.request_id = req.id;
        "
        )
        // let result = sqlx::query_as!(DBRow, "SELECT content FROM request")
        .fetch_all(&self.pool)
        .await?;

        Ok(result
            .into_iter()
            .map(|x| ReqRes::try_from(x))
            .collect::<anyhow::Result<Vec<_>>>()?)

    }

    // pub async fn get_recent_responses(&self) -> anyhow::Result<Vec<ResponseData>> {
    //     let result = sqlx::query_as!(DBRow, "SELECT uuid, content FROM response")
    //     // let result = sqlx::query_as!(DBRow, "SELECT content FROM request")
    //         .fetch_all(&self.pool)
    //         .await?;
    //     Ok(result.into_iter().map(|x| ResponseData::try_from(x)).collect::<anyhow::Result<Vec<_>>>()?)
    // }
}


#[async_trait::async_trait]
pub trait RequestStorage {
    async fn store_request(&mut self, req: &RequestData) -> anyhow::Result<u64>;
    async fn store_response(&mut self, req: &ResponseData) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl RequestStorage for DB {
    async fn store_request(&mut self, req: &RequestData) -> anyhow::Result<u64> {
        self.insert_request(req).await
    }
    async fn store_response(&mut self, res: &ResponseData) -> anyhow::Result<()> {
        self.insert_response(res).await
    }
}

#[async_trait::async_trait]
impl RequestStorage for Arc<DB> {
    async fn store_request(&mut self, req: &RequestData) -> anyhow::Result<u64> {
        self.insert_request(req).await
    }
    async fn store_response(&mut self, res: &ResponseData) -> anyhow::Result<()> {
        self.insert_response(res).await
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
