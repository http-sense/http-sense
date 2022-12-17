#![feature(file_create_new)]
mod cli;
mod config;
mod supabase;
mod socketio;
mod db;
mod model;
mod proxy_server;
mod ui_server;
use anyhow::Context;
use clap::Parser;
use db::RequestStorage;
use futures::never::Never;
use proxy_server::ProxyEvent;

use crate::{config::get_database_file, db::DB};

use std::{sync::Arc, collections::HashMap};

#[async_trait::async_trait(?Send)]
trait EventConsumer {
    async fn consume(&mut self, mut rx: tokio::sync::broadcast::Receiver<ProxyEvent>) -> anyhow::Result<()>;
}


#[async_trait::async_trait(?Send)]
impl<T: RequestStorage> EventConsumer for T {
    async fn consume(&mut self, mut rx: tokio::sync::broadcast::Receiver<ProxyEvent>) -> anyhow::Result<()> {
        tracing::info!("Consumption Started");
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
                    unimplemented!()
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut db = DB::connect(&get_database_file()?).await?;
    let mut shared_db = Arc::new(db);
    let proxy_db = shared_db.clone();
    let ui_db = shared_db.clone();
    let args = cli::CLIArgs::parse();

    let (tx, mut rx) = tokio::sync::broadcast::channel(128);
    tokio::select! {
        v = proxy_server::start_server(tx, args.proxy_port, &args.proxy_addr, &args.origin_url) => {
            tracing::error!("Proxy server has stopped");
            v?;
        }
        j = ui_server::start_server(ui_db, args.ui_port, &args.ui_addr) => {
            tracing::error!("UI server has stopped");
            j?;
        }
        _ = shared_db.consume(rx) => {
            tracing::error!("DB Consumer has stopped");
        }
    };

    Ok(())
}
