#![feature(file_create_new, async_closure)]
mod cli;
mod config;
mod supabase;
mod socketio;
mod db;
mod model;
mod proxy_server;
mod ui_server;
mod supabase_auth;
use anyhow::Context;
use clap::Parser;
use db::RequestStorage;
use futures::never::Never;
use proxy_server::ProxyEvent;
use tracing::info;

use crate::{config::{get_database_file, SUPABASE_PROJECT_URL, SUPABASE_ANON_KEY}, db::DB, supabase::SupabaseDb, supabase_auth::create_user};

use std::{sync::Arc, collections::HashMap, pin::Pin};

#[async_trait::async_trait(?Send)]
trait EventConsumer {
    async fn consume(&mut self, mut rx: tokio::sync::broadcast::Receiver<ProxyEvent>, consumer_name: &str) -> anyhow::Result<()>;
}

async fn infinite_sleep() -> anyhow::Result<()> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(100000.)).await;
    }
}

#[async_trait::async_trait(?Send)]
impl<T: RequestStorage> EventConsumer for T {
    async fn consume(&mut self, mut rx: tokio::sync::broadcast::Receiver<ProxyEvent>, consumer_name: &str) -> anyhow::Result<()> {
        tracing::info!("Consumer attached: {}", consumer_name);
        let mut requests: HashMap<uuid::Uuid, u64> = HashMap::new();
        loop {
            let value = rx.recv().await?;

            match value {
                ProxyEvent::ResponseRecv(rid, res) => {
                    let store_id = requests.remove(&rid).context("Got response without request")?;
                    self.store_response(store_id, &res).await?
                },
                ProxyEvent::RequestRecv(rid, req) => {
                    let store_id = self.store_request(&req).await?;
                    requests.insert(rid, store_id);

                },
                ProxyEvent::RequestError(rid, error) => {
                    let store_id = requests.remove(&rid).context("Got Error without request")?;
                    tracing::error!("Couldn't finish the request. Error: {:?}", &error);
                    self.store_error(store_id, &error).await?;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    log::set_max_level(log::LevelFilter::Off);

    let mut db = DB::connect(&get_database_file()?).await?;
    let mut shared_db = Arc::new(db);
    let proxy_db = shared_db.clone();
    let ui_db = shared_db.clone();
    let args = cli::CLIArgs::parse();
    let (tx, mut rx) = tokio::sync::broadcast::channel(128);
    let mut rx2 = tx.subscribe();
    let mut supabase_db = None;
    if (args.publish) {
        let user = create_user().await?;
        let ticket = base64::encode(format!("{}::{}", &user.email, &user.password));
        let url = url::Url::parse_with_params("https://nkit.dev/zoo", &[("ticket", ticket)]).unwrap();

        let sup_db = SupabaseDb::new(SUPABASE_PROJECT_URL, SUPABASE_ANON_KEY, user);
        supabase_db = Some(sup_db);
        info!("Request logs published at: {}", url.to_string());
    }

    let publish_future = if let Some(sup_db)  = supabase_db.as_mut() {
        sup_db.consume(rx2, "supabase_db")
    } else {
        // Hack to get around 
        Box::pin(infinite_sleep())
    };
    let origin_url = crate::cli::to_url(&args.origin_url).context(format!("Origin Url is not valid: {}", &args.origin_url))?;
    

    tokio::select! {
        v = proxy_server::start_server(tx, args.proxy_port, &args.proxy_addr, origin_url) => {
            tracing::error!("Proxy server has stopped");
            v?;
        }
        j = ui_server::start_server(ui_db, args.ui_port, &args.ui_addr) => {
            tracing::error!("UI server has stopped");
            j?;
        }
        e = shared_db.consume(rx, "local_db") => {
            tracing::error!("DB Consumer has stopped {:?}", e);
        }
        r = publish_future => {
            tracing::error!("Publishing stopped {:?}", r);
        }
    };

    Ok(())
}
