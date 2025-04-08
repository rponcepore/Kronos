//! startup.rs

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

// The modules we wrote, that we will use here
// use database::run_database; // means reference run_database as just "run_database""
use crate::routes::api::api_handler::api_handler;
use crate::routes::healthchecks::database_health_check;
use crate::routes::healthchecks::health_check;
use crate::routes::healthchecks::health_check_body;

/*
 * The main driver function of the entire application.
 */
pub fn run_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let clone = listener.try_clone().unwrap();
    let server = HttpServer::new(|| {
        App::new()
            // Adding some middleware. I am allowing these explicitly for now until I learn more.
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:9000") // Only allow this origin (React App)
                    .allowed_origin("http://localhost:5173") // (Other react app)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["Content-Type"]), // Allow specific headers
            )
            .route("/healthcheck", web::get().to(health_check))
            .route("/health_check", web::get().to(health_check))
            .route("/health_check_body", web::get().to(health_check_body))
            .route(
                "/database_health_check",
                web::get().to(database_health_check),
            )
            .route("/api", web::post().to(api_handler))
    })
    .listen(listener)?
    .run();
    println!("Server initialization successful.");
    println!(
        "Server is located at {}:{}",
        clone.local_addr().unwrap().ip(),
        clone.local_addr().unwrap().port()
    );
    Ok(server)
}
