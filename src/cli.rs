use clap::Parser;
use ansi_term::Style;

type Port = u16;
type Addr = String;

fn create_about() -> String {
   let bold_style = Style::new().bold();
   let bold_under_style = Style::new().bold().underline();
   let banner = "
+-+-+-+-+ +-+-+-+-+-+
|H|T|T|P| |S|E|N|S|E|
+-+-+-+-+ +-+-+-+-+-+
";
   format!("
{banner}
Make sense of what is coming and what is leaving your http server.

{tldr_title}:
   http-sense http://localhost:8004 --publish
   http-sense http://localhost:8004 --proxy-port 6001 --publish
   http-sense http://localhost:8004 --proxy-port 6001 --proxy-addr 0.0.0.0

", banner=bold_style.paint(banner), tldr_title=bold_under_style.paint("TLDR"))
}

#[derive(Parser, Debug)]
// #[command(author, version, about, long_about = ABOUT)]
#[command(author, version, about, long_about = create_about())]
pub struct CLIArgs {
   pub origin_url: String,

   #[arg(long, default_value_t=false, help="Publish requests to supabase db, allowing you to remotely access request details")]
   pub publish: bool,

   #[arg(short='p', long, default_value_t=6100, help="Port at which proxy server should listen")]
   pub proxy_port: Port,

   #[arg(short='a', long, default_value_t={"127.0.0.1".to_string()}, help="Address that proxy server should bind to")]
   pub proxy_addr: Addr,


   #[arg(long, default_value_t=6101, help="Port at which ui server should listen (Alpha)")]
   pub ui_port: Port,

   #[arg(long, default_value_t={"127.0.0.1".to_string()}, help="Address that ui server should bind to")]
   pub ui_addr: Addr,
}