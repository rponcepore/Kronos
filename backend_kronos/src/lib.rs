//! lib.rs
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

/*
 * This is the healthcheck handler
 * @param req: must be a HttpRequest, GET
 * @return 200 OK with no body
 */
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

/*
 * The main driver function of the entire application.
 */
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .listen(listener)?
        .run();
    Ok(server)
}