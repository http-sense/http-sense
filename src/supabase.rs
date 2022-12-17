use crate::{db::RequestStorage, model::{RequestData, ResponseData}};


pub struct SupabaseDb {
	
}

#[async_trait::async_trait]
impl RequestStorage for SupabaseDb {
	async fn store_request(&mut self, req: &RequestData) -> anyhow::Result<u64> {
		todo!()
	}

	async fn store_response(&mut self, res: &ResponseData) -> anyhow::Result<()> {
		todo!()
	}
}
