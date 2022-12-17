use crate::{db::RequestStorage, model::{RequestData, ResponseData}};
use postgrest::Postgrest;
use serde_json::json;


pub struct SupabaseDb {
	connection: Postgrest
}

impl SupabaseDb {
	fn new(endpoint: &str) -> Self {
		Self {
			connection: Postgrest::new(endpoint)
		}
	}
}

#[async_trait::async_trait]
impl RequestStorage for SupabaseDb {
	async fn store_request(&mut self, req: &RequestData) -> anyhow::Result<u64> {
		let result = self.connection.from("request").insert(
			serde_json::to_string(&json!( {
					"a": 3
				}))?
			).execute().await?;
		todo!()
	}

	async fn store_response(&mut self, res: &ResponseData) -> anyhow::Result<()> {
		todo!()
	}
}

#[test]
fn my_test() {
	let endpoint = "https://wfeoffbfmtjjzamwjlob.supabase.co";
	let db = SupabaseDb::new(endpoint);
	// db.
}