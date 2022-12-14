use clap::Parser;

type Port = u16;
type Addr = String;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
   pub origin_url: String,

   #[arg(short='p', long, default_value_t=6100)]
   pub proxy_port: Port,

   #[arg(short='a', long, default_value_t={"127.0.0.1".to_string()})]
   pub proxy_addr: Addr,

   #[arg(long, default_value_t=6101)]
   pub ui_port: Port,

   #[arg(long, default_value_t={"127.0.0.1".to_string()})]
   pub ui_addr: Addr,
}