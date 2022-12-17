use std::str::FromStr;

use crate::{db::RequestStorage, model::{RequestData, ResponseData}};
use anyhow::Context;
use bytes::Bytes;
use postgrest::Postgrest;
use serde_json::json;


pub struct SupabaseDb {
	connection: Postgrest
}

impl SupabaseDb {
	fn new(endpoint: &str, service_key: &str) -> Self {
		Self {
			connection: Postgrest::new(endpoint)
				.insert_header("apikey", service_key)
				.insert_header("Authorization", format!("Bearer {}", service_key))
		}
	}
}

impl SupabaseDb {
	async fn insert_in_table(&mut self, table_name: &str, content: &serde_json::Value) -> anyhow::Result<u64> {
		let r = self.connection.from(table_name).insert(
			serde_json::to_string(content)?
		).execute().await?;

		if (!r.status().is_success()) {
			anyhow::bail!("bad response from supabase {:?}", r)
		}

		let response = r.text().await?;
		let k = serde_json::Value::from_str(&response)?;
		// dbg!(&response, &k);
		Ok(k.get(0).context("Unexpected empty row")?.get("id").context("id not present")?.as_u64().context("id is not u64")?)
	}
}

#[async_trait::async_trait]
impl RequestStorage for SupabaseDb {
	async fn store_request(&mut self, req: &RequestData) -> anyhow::Result<u64> {
		self.insert_in_table("request", &json!({
			"content": serde_json::to_string(req)?
		})).await
	}

	async fn store_response(&mut self, res: &ResponseData) -> anyhow::Result<()> {
		self.insert_in_table("reseponse", &json!({
			"content": serde_json::to_string(res)?,
			"request_id": res.request_id
		})).await?;
		Ok(())
	}
}

#[tokio::test]
async fn my_test() {
	return ();
	let endpoint = "https://wfeoffbfmtjjzamwjlob.supabase.co/rest/v1";
	let service_key = "Bring you own key!!!!";
	let mut db = SupabaseDb::new(endpoint, service_key);
	let r = db.store_request(&RequestData {
		headers: Default::default(),
		uri: "/hello".parse().unwrap(),
		method: http::Method::GET,
		body: Bytes::from_static(b"Just checking")
	}).await;
	dbg!(r);

	assert_eq!(1, 2);
}