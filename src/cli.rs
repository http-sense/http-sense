use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {

   #[arg(short, long)]
   origin_url: String,

   #[arg(short, long, default_value_t=6100)]
   proxy_port: u32,

   #[arg(short, long, default_value_t={"127.0.0.1".to_string()})]
   proxy_addr: String,

   #[arg(short, long, default_value_t=6101)]
   ui_port: u32,

   #[arg(short, long, default_value_t={"127.0.0.1".to_string()})]
   ui_addr: String,
}