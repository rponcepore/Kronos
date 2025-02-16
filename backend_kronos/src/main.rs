use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};

/*
 * This is a request handler. It is disused.
 * @param req: type HttpRequest
 * @return: impl Responder (in this case, String)
 */

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

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
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
