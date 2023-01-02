#![feature(file_create_new, async_closure)]
mod api_server;
mod cli;
mod config;
mod db;
mod models;
mod proxy_server;
mod socketio;
mod axum_utils;
mod bytes_util;
#[macro_use]
mod prelude;
mod serde_util;
mod supabase;


mod supabase_auth;
use anyhow::Context;
use clap::Parser;
use db::RequestStorage;
use models::Request;
use proxy_server::{RequestEvent, ProxyEvent};



use crate::{
    config::{get_database_file, SUPABASE_ANON_KEY, SUPABASE_PROJECT_URL},
    db::DB,
    supabase::SupabaseDb,
    supabase_auth::create_user,
};

use std::{collections::HashMap, sync::Arc, hash::Hash};

#[async_trait::async_trait(?Send)]
trait EventConsumer {
    async fn consume(
        &mut self,
        mut rx: tokio::sync::broadcast::Receiver<ProxyEvent>,
        mut tx: Option<tokio::sync::broadcast::Sender<RequestEvent<u64>>>,
        consumer_name: &str,
    ) -> anyhow::Result<()>;
}

async fn infinite_sleep() -> anyhow::Result<()> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(100000.)).await;
    }
}

#[async_trait::async_trait(?Send)]
impl<T: RequestStorage> EventConsumer for T {
    async fn consume(
        &mut self,
        mut rx: tokio::sync::broadcast::Receiver<ProxyEvent>,
        tx: Option<tokio::sync::broadcast::Sender<RequestEvent<u64>>>,
        _consumer_name: &str,
    ) -> anyhow::Result<()> {
        let mut requests: HashMap<u64, u64> = HashMap::new();
        loop {
            let value = rx.recv().await?;

            match value {
                RequestEvent::Response(rid, res) => {
                    let store_id = requests
                        .remove(&rid)
                        .context("Got response without request")?;
                    self.store_response(store_id, &res).await?;
                    if let Some(tx) = tx.as_ref() {
                        tx.send(RequestEvent::Response(store_id, res))?;
                    };
                }
                RequestEvent::Request(rid, req) => {
                    let store_id = self.store_request(&req).await?;
                    requests.insert(rid, store_id);
                    if let Some(tx) = tx.as_ref() {
                        tx.send(RequestEvent::Request(store_id, req))?;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    log::set_max_level(log::LevelFilter::Off);
    println!("Starting: \n");

    let db = DB::connect(&get_database_file()?).await?;
    let mut shared_db = Arc::new(db);
    let _proxy_db = shared_db.clone();
    let ui_db = shared_db.clone();
    let args = cli::CLIArgs::parse();

    // Proxy Event Queue
    let (tx, rx) = tokio::sync::broadcast::channel(128);
    let rx2 = tx.subscribe();
    let rx3 = tx.subscribe();

    // Db Events
    let (db_tx, db_rx) = tokio::sync::broadcast::channel(128);

    let mut supabase_db = None;
    if args.publish {
        let user = create_user().await?;
        let ticket = base64::encode(format!("{}::{}", &user.email, &user.password));
        let uuid = user.uid();
        let base_url = format!("https://www.httpsense.com/{uuid}/#{ticket}");
        let url = url::Url::parse(&base_url).unwrap();

        let sup_db = SupabaseDb::new(SUPABASE_PROJECT_URL, SUPABASE_ANON_KEY, user);
        supabase_db = Some(sup_db);

        let title = ansi_term::Style::new().bold();
        println!(
            "   {} -> {}\n",
            title.paint("Dashboard Url"),
            url.to_string()
        );
    }

    let publish_future = if let Some(sup_db) = supabase_db.as_mut() {
        sup_db.consume(rx2, None, "supabase_db")
    } else {
        // Hack to get around
        Box::pin(infinite_sleep())
    };
    let origin_url = crate::cli::to_url(&args.origin_url)
        .context(format!("Origin Url is not valid: {}", &args.origin_url))?;

    tokio::select! {
        v = proxy_server::start_server(tx, args.proxy_port, &args.proxy_addr, origin_url) => {
            tracing::error!("Proxy server has stopped");
            v?;
        }
        j = api_server::start_server(ui_db, args.api_port, &args.api_addr, db_rx) => {
            tracing::error!("UI server has stopped");
            j?;
        }
        e = shared_db.consume(rx, Some(db_tx), "local_db") => {
            tracing::error!("DB Consumer has stopped {:?}", e);
        }
        r = publish_future => {
            tracing::error!("Publishing stopped {:?}", r);
        }
    };

    Ok(())
}
