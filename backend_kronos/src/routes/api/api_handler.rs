//! api.rs

// This file defines the api for calls from the frontend.

use actix_web::{
    web,
    http::{header::ContentType, StatusCode}, 
    HttpRequest, HttpResponse, Responder};
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct KronosRequest{
    pub http_method: String,
    pub action: String,
    pub unit: String,
}

/*
 * Core API call handler for the application, matching JSON "action" parameter 
 * to the correct function call.
 * This handler can only be called if the content type of the http request (in the header!) is JSON,
 * and the content of the request can be deserialized to a "KronosRequest" struct
 * @param req: must be a HttpRequest, GET, with a JSON body with parameters.
 * @return 200 OK with JSON body
 */
pub async fn api_handler(kronos_request_as_json: Result<web::Json<KronosRequest>, actix_web::Error>) -> impl Responder {

    match kronos_request_as_json {
        Ok(req) => {
            println!("api_handler called, request body: {:?}", req);
            let http_method = &req.http_method;
            let action = &req.action;
            let unit = &req.unit;

            println!("Method: {}, Action: {}, Unit: {}", http_method, action, unit);
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            println!("API handler called, but failed to deserialize JSON: {}", err);
            HttpResponse::BadRequest().body(format!("Invalid JSON: {}\n", err))
        }
    }
    
}



