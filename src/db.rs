
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
    uri: String
}


impl TryFrom<DBRow> for RequestData {
    type Error = anyhow::Error;
    fn try_from(value: DBRow) -> anyhow::Result<Self> {
        Ok(Self {
            uri: value.uri.parse()?
        })
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
                    uri TEXT NOT NULL
                )
            "
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_request(&self, req: &RequestData) -> anyhow::Result<()> {
        let uri = req.uri.to_string();
        sqlx::query!("INSERT INTO request (uri) VALUES (?)", uri)
            .fetch_all(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_recent_requests(&self) -> anyhow::Result<Vec<RequestData>> {
        let result = sqlx::query_as!(DBRow, "SELECT uri FROM request")
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(|x| RequestData::try_from(x)).collect::<anyhow::Result<Vec<_>>>()?)
    }
}

#[cfg(test)]
mod tests {
    use http::Uri;

    use super::*;
    use std::env;

    #[tokio::test]
    async fn inserts_should_work() -> anyhow::Result<()> {
        let tmpdir = tempfile::tempdir().unwrap();
        env::set_current_dir(&tmpdir).unwrap();

        let b = std::fs::File::create("test.sqlite");
        drop(b);

        let mut db = DB::connect("sqlite://test.sqlite").await?;
        let results = db.get_recent_requests().await?;
        assert!(results.is_empty());
        db.insert_request(&RequestData {uri: "JustChecking".parse().unwrap()}).await?;

        let results = db.get_recent_requests().await?;
        assert_eq!(results.len(),  1);
        assert_eq!(results[0].uri,  "JustChecking".parse::<Uri>().unwrap());

        Ok(())
    }

}