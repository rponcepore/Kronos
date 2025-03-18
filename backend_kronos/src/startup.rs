//! startup.rs

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

// The modules we wrote, that we will use here
// use database::run_database; // means reference run_database as just "run_database""
use crate::routes::healthchecks::health_check;
use crate::routes::healthchecks::database_health_check;
use crate::routes::healthchecks::health_check_body;
use crate::routes::api::api_handler::api_handler;

/*
 * The main driver function of the entire application.
 */
pub fn run_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let clone = listener.try_clone().unwrap();
    let server = HttpServer::new(|| {
            App::new()
                .route("/healthcheck", web::get().to(health_check))
                .route("/health_check", web::get().to(health_check))
                .route("/health_check_body", web::get().to(health_check_body))
                .route("/database_health_check", web::get().to(database_health_check))
                .route("/api", web::post().to(api_handler))
        })
        .listen(listener)?
        .run();
    println! ("Server initialization successful.");
    println! ("Server is located at {}:{}", clone.local_addr().unwrap().ip(), clone.local_addr().unwrap().port());
    Ok(server)
}