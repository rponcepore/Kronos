//! lib.rs
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

// Module tree declarations (I don't like this "feature" yet)
mod database; // effectively means "import the module called database"
mod handlers; // points to our handlers routines
pub mod environment; 


// The modules we wrote, that we will use here
// use database::run_database; // means reference run_database as just "run_database""
use crate::handlers::healthchecks::health_check;
use crate::handlers::healthchecks::database_health_check;
use crate::handlers::healthchecks::health_check_body;

/*
 * The main driver function of the entire application.
 */
pub fn run_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/healthcheck", web::get().to(health_check))
                .route("/health_check", web::get().to(health_check))
                .route("/health_check_body", web::get().to(health_check_body))
                .route("/database_health_check", web::get().to(database_health_check))
        })
        .listen(listener)?
        .run();
    println! ("Server initialization successful.");
    Ok(server)
}