//! healthchecks.rs

// This file contains the various healthcheck routes employed by the server. 

use actix_web::{
    http::{header::ContentType, StatusCode}, 
    HttpRequest, HttpResponse, Responder};
use sea_orm::*;
use debug_print::debug_println as dprintln;

use crate::configuration::get_configuration;

/*
 * This is the healthcheck handler for the webserver only
 * @param req: must be a HttpRequest, GET
 * @return 200 OK with no body
 */
pub async fn health_check(_req: HttpRequest) -> impl Responder {
    dprintln!("health_check called!");
    HttpResponse::Ok().finish()
}

/*
 * This is the healthcheck handler for the webserver only
 * @param req: must be a HttpRequest, GET
 * @return 200 OK with no body
 */
pub async fn health_check_body(_req: HttpRequest) -> impl Responder {
    dprintln!("health_check_body called!");
    HttpResponse::Ok().body("health_check_body success!")
}

/*
 * This is the healthcheck handler that includes both webserver and database.
 * @param req: must be a HttpRequest, GET
 * @return 200 OK with no body
 */
pub async fn database_health_check(_req: HttpRequest) -> impl Responder {
    dprintln!("database_health_check called!");
    let configuration = get_configuration().expect("Failed to read configuration.");
    dprintln!("Configuration read successfully.");
    let connection_string = configuration.database.connection_string();
    dprintln!("Connection string: {}", connection_string);
    match Database::connect(connection_string).await {
        Ok(..) => {
            // Connection successful
            dprintln!("Successfully connected to the database.");
            return HttpResponse::Ok().finish();
        }
        Err(err) => {
            // Connection failed
            dprintln!("Failed to connect to the database: {}", err);
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .insert_header(ContentType::html())
                .body(format!("Failed to connect to the database: {}", err))
        }
    };
}

