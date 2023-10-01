use axum::{routing::get, Router};
use clap::{Parser, Subcommand};
use std::{path::PathBuf, net::SocketAddr};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli{
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Server{
        #[clap(short = 'a', long = "addr", default_value = "0.0.0.0")]
        addr: String,
        #[clap(short = 'p', long = "port", default_value = "3000")]
        port: u16
    },
}

async fn index() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Server { addr, port }) => {
            let socket_addr: SocketAddr = format!("{}:{}", addr, port).parse().expect("error parsing address and port");

            let app = Router::new().route("/", get(index));

            axum::Server::bind(&socket_addr).serve(app.into_make_service()).await.unwrap();
        }
        None => ()
    }
}