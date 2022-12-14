#![feature(file_create_new)]
mod cli;
mod config;
mod db;
mod model;
mod proxy_server;
mod ui_server;
use clap::Parser;

use crate::{config::get_database_file, db::DB};

use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db = DB::connect(&get_database_file()?).await?;
    let shared_db = Arc::new(db);
    let proxy_db = shared_db.clone();
    let args = cli::CLIArgs::parse();

    proxy_server::start_proxy(proxy_db, args.proxy_port, &args.proxy_addr).await?;

    Ok(())
}
