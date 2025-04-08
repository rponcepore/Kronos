//! api.rs

// This file defines the api for calls from the frontend.
use actix_web::web;
use actix_web::{web::Json, HttpResponse, Responder};

use debug_print::debug_println as dprintln;

// Pull in our entity Summaries

// Include our database configs
use crate::routes::api::api_methods::{
    orders_api::{create_order::*, get_order::*},
    paragraph_api::{delete_paragraph::*, edit_paragraph::*, insert_paragraph::*},
    plans_api::{create_plan::*, get_plans::*},
};
use crate::routes::api::parameters::network_structs::*;

/*
 * Core API call handler for the application, matching JSON "api_method" parameter
 * to the correct function call.
 * This handler can only be called if the content type of the http request (in the header!) is JSON,
 * and the content of the request can be deserialized to a "KronosRequest" struct
 * @param req: must be a HttpRequest, GET, with a JSON body with parameters.
 * @return 200 OK with JSON body
 */
pub async fn api_handler(
    kronos_request_as_json: Result<web::Json<KronosRequest>, actix_web::Error>,
) -> impl Responder {
    // Check for a valid request and that uic and api_method are not null.
    let valid_req: Json<KronosRequest> = match kronos_request_as_json {
        Ok(req) => match is_request_valid(req).await {
            Ok(valid_req) => valid_req,
            Err(bad_request_http_response) => return bad_request_http_response,
        },
        Err(msg) => return HttpResponse::BadRequest().body(format!("Invalid JSON: {}\n", msg)),
    };

    // a valid_req has no null values for either api_method or uic, so we can unwrap without worry.
    // Also, even though unwrap() will panic (and crash the program) if it fails, I want to crash
    // because it means my is_valid_request function failed.
    let api_method = valid_req.api_method.as_ref().unwrap().as_str();

    let kronos_response: Result<KronosResponse, KronosApiError> = match api_method {
        // Plans actions
        "create_plan" => create_plan(valid_req).await,
        "get_plans" => get_plans(valid_req).await,
        // Orders actions
        "get_order" => get_order(valid_req).await,
        "create_order" => create_order(valid_req).await,
        // Paragraph actions
        "get_paragraph" => Err(KronosApiError::NotImplemented(
            "get_paragraph not implemented.".to_string(),
        )),
        "insert_paragraph" => insert_paragraph(valid_req).await,
        "edit_paragraph" => edit_paragraph(valid_req).await,
        "delete_paragraph" => delete_paragraph(valid_req).await,
        // Return a BadRequest response if the api_method was invalid.
        _ => {
            return HttpResponse::BadRequest().body(format!("Invalid api_method: {}\n", api_method))
        }
    };

    match kronos_response {
        Ok(kronos_response) => {
            dprintln!("{:?}", kronos_response);
            HttpResponse::Ok().json(kronos_response)
        }
        Err(kronos_api_error) => match kronos_api_error {
            KronosApiError::DbErr(err) => {
                HttpResponse::InternalServerError().body(format!("Database failure: {}\n", err))
            }
            KronosApiError::ActixError(err) => HttpResponse::InternalServerError()
                .body(format!("Internal server error: {}\n", err)),
            KronosApiError::NotImplemented(msg) => {
                HttpResponse::NotImplemented().body(format!("Not implemented: {}\n", msg))
            }
            KronosApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().body(format!("Bad request to API: {}\n", msg))
            }
            KronosApiError::ExpectedDataNotPresent(msg) => HttpResponse::InternalServerError()
                .body(format!("Expected data not found in database: {}\n", msg)),
            KronosApiError::Unknown(msg) => {
                HttpResponse::BadRequest().body(format!("Unknown error: {}\n", msg))
            }
        },
    }
}

// Invariant: Valid requests always have, at a minimum, a uic and an api_method.
// This method occurs AFTER deserialization is proven successful.
async fn is_request_valid(req: Json<KronosRequest>) -> Result<Json<KronosRequest>, HttpResponse> {
    // Deserialization successful
    dprintln!("api_handler called, request body: {:?}", req);

    // First, check if the api_method is null
    if req.api_method.is_none() {
        return Err(HttpResponse::BadRequest().body(format!("Request api_method field is null.")));
    }

    if req.uic.is_none() {
        return Err(HttpResponse::BadRequest().body(format!("Request uic field is null.")));
    }

    return Ok(req);
}

impl KronosRequest {
    pub fn new() -> Self {
        Self {
            api_method: None,
            uic: None,
            plan_request: None,
            order_request: None,
            paragraph_request: None,
            task_request: None,
        }
    }

    pub fn with_action(mut self, api_method: String) -> Self {
        self.api_method = Some(api_method);
        self
    }

    pub fn with_unit(mut self, uic: String) -> Self {
        self.uic = Some(uic);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
