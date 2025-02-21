//! main.rs

use backend_kronos::run_server; // main automatically can see lib.rs files
use backend_kronos::environment::*;
use std::net::TcpListener;
use clap::Parser;

// Imports that I wrote
use backend_kronos::configuration::get_configuration;

/*
 * The entrypoint of the entire application.
 */
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Read in our configuration settings.
    let configuration = get_configuration().expect("Failed to read configuration.");

    let kargs:Kargs = Kargs::parse(); 
    let kenv = match parse_args(kargs) {
        Ok(config) => config,
        Err(err) => {
            println! ("{:?}", err);
            println!("Argument parsing failure. At a minimum, you must supply --build dev or --build prod");
            std::process::exit(1);
            }
    };
    
    let address = kenv.server_address;
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind(&address)
        .expect("Failed to bind port 8000");
    run_server(listener)?.await
}
