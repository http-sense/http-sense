
use serde::Serialize;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnection;
use sqlx::sqlite::SqliteExecutor;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Connection;
use crate::model::RequestData;

#[derive(Debug, Clone)]
pub struct DB {
    pool: SqlitePool,
}

struct DBRow {
    uuid: String,
    content: String
}


impl TryFrom<DBRow> for RequestData {
    type Error = anyhow::Error;
    fn try_from(value: DBRow) -> anyhow::Result<Self> {
        let result: Self = serde_json::from_str(&value.content)?;
        Ok(result)
        // value.content.into
        // Ok(Self {
        //     uuid: value.uuid.parse()?
        // })
    }
}

impl DB {
    pub async fn connect(db_file: &str) -> anyhow::Result<Self> {
        let mut db = Self {
            pool: SqlitePoolOptions::new().max_connections(3).connect(db_file).await?,
        };
        db.bootstrap().await?;
        Ok(db)
    }

    async fn bootstrap(&self) -> anyhow::Result<()> {
        sqlx::query!(
            "
                CREATE TABLE IF NOT EXISTS request (
                    id INTEGER PRIMARY KEY,
                    uuid TEXT NOT NULL,
                    content TEXT NOT NULL
                )
            "
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_request(&self, req: &RequestData) -> anyhow::Result<()> {
        // tracing::info!("Inserting in db");
        // let uri = req.uri.to_string();
        // let serializer = serde::Serializer

        // // Headers
        // let mut writer = Vec::with_capacity(128);
        // let mut ser = serde_json::Serializer::new(writer);
        // let r = http_serde::header_map::serialize(&req.headers, &mut ser);
        // let header_serialized = unsafe {
        //     // We do not emit invalid UTF-8.
        //     String::from_utf8_unchecked(writer)
        // };

        // // Method
        // let mut writer = Vec::with_capacity(128);
        // let mut ser = serde_json::Serializer::new(writer);
        // http_serde::method::serialize(&req.method, &mut ser)?;
        // let method_serialized = unsafe {
        //     // We do not emit invalid UTF-8.
        //     String::from_utf8_unchecked(writer)
        // };

        // UUid
        // let mut writer = Vec::with_capacity(128);
        // let mut ser = serde_json::Serializer::new(&mut writer);
        // req.uuid.serialize(&mut ser)?;
        // // http_serde::method::serialize(&req.uuid, &mut ser)?;
        // let uuid_ser = unsafe {
        //     // We do not emit invalid UTF-8.
        //     String::from_utf8_unchecked(writer)
        // };

        // Method
        let uuid_ser = serde_json::to_string(&req.uuid)?;
        let content = serde_json::to_string(req)?;
        dbg!(&uuid_ser, &content);


        // http_serde::header_map::serialize(&req.headers, ser)
        sqlx::query!("INSERT INTO request (uuid, content) VALUES (?, ?)", uuid_ser, content)
            .fetch_all(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_recent_requests(&self) -> anyhow::Result<Vec<RequestData>> {
        let result = sqlx::query_as!(DBRow, "SELECT uuid, content FROM request")
        // let result = sqlx::query_as!(DBRow, "SELECT content FROM request")
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(|x| RequestData::try_from(x)).collect::<anyhow::Result<Vec<_>>>()?)
    }
}

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
