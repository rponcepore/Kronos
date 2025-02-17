//! main.rs
use backend_kronos::run;
use std::net::TcpListener;
/*
 * The entrypoint of the entire application.
 */
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind port 8000");
    run(listener)?.await
}
