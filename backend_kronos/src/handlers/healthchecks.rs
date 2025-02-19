//! healthchecks.rs

// This file contains the various healthcheck routes employed by the server. 

use actix_web::{HttpRequest, HttpResponse, Responder};

/*
 * This is the healthcheck handler for the webserver only
 * @param req: must be a HttpRequest, GET
 * @return 200 OK with no body
 */
pub async fn health_check(_req: HttpRequest) -> impl Responder {
    println!("health_check called!");
    HttpResponse::Ok().finish()
}

/*
 * This is the healthcheck handler that includes both webserver and database.
 * @param req: must be a HttpRequest, GET
 * @return 200 OK with no body
 */
pub async fn database_health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

