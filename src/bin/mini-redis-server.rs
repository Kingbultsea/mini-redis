//! mini-redis server.
//!
//! This file is the entry point for the server implemented in the library. It
//! performs command line parsing and passes the arguments on to
//! `mini_redis::server`.
//!
//! The `clap` crate is used for parsing arguments.

use mini_redis::server;

use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    // Bind a TCP listener
    let listener = TcpListener::bind(&format!("127.0.0.1:6379")).await?;

    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}
