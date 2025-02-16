//! main.rs
use backend_kronos::run;

/*
 * The entrypoint of the entire application.
 */
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run()?.await
}
