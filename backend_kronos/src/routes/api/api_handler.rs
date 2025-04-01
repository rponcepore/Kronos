//! api.rs

// This file defines the api for calls from the frontend.
use actix_web::web;
use actix_web::{
    web::Json,
    HttpResponse, 
    Responder
    };
    
use sea_orm::*;
use serde::Serialize;
use debug_print::debug_println as dprintln;

// Pull in our entity Summaries

use crate::models::entity_summaries::plan_summary::PlanSummary;
use crate::models::entity_summaries::kronos_order_summary::KronosOrderSummary;
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;
use crate::models::entity_summaries::unit_summary::UnitSummary;

// Include our database configs
use crate::configuration::get_configuration;
use crate::routes::api::api_methods::get_plans::get_plans;
use crate::routes::api::api_methods::get_order::get_order;

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct KronosRequest {
    //pub request_id: Integer,
    //pub http_method: Option<String>,
    pub action: Option<String>,
    pub unit: Option<String>,
    pub order_id: Option<i32>,
    pub paragraph_id: Option<i32>,
    pub task_id: Option<i32>,
}

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct KronosResponse {
    pub kronos_request: KronosRequest,
    pub plans_vec: Option< Vec<PlanSummary>>,
    pub orders_vec: Option< Vec<KronosOrderSummary>>,
    pub paragraphs_vec: Option< Vec<ParagraphSummary>>,
    pub units_vec: Option< Vec<UnitSummary>>,
}

pub enum KronosApiError  {
    DbErr(sea_orm::DbErr),
    ActixError(actix_web::Error),
    NotImplemented(String),
    BadRequest(String),
    ExpectedDataNotPresent(String),
    Unknown(String),
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

    // Check for a valid request and that unit and action are not null.
    let valid_req: Json<KronosRequest> = match kronos_request_as_json{
        Ok(req) => match is_request_valid(req).await{
            Ok(valid_req) => valid_req,
            Err(bad_request_http_response) => return bad_request_http_response,
        },
        Err(msg) => return HttpResponse::BadRequest().body(format!("Invalid JSON: {}\n", msg)),
    };

    // a valid_req has no null values for either action or unit, so we can unwrap without worry.
    // Also, even though unwrap() will panic (and crash the program) if it fails, I want to crash
    // because it means my is_valid_request function failed.
    let action = valid_req.action.as_ref().unwrap().as_str();

    let kronos_response: Result<KronosResponse, KronosApiError> = match action {
        "get_plans" => get_plans(valid_req).await,
        "get_order" => get_order(valid_req).await,
        "update_paragraph" => Err(KronosApiError::NotImplemented("update_paragraph not implemented.".to_string())),
        // Return a BadRequest response if the action was invalid.
        _ => return HttpResponse::BadRequest().body(format!("Invalid action: {}\n", action)),
    };

    match kronos_response {
        Ok(kronos_response) => {
            dprintln!("{:?}", kronos_response);
            HttpResponse::Ok().json(kronos_response)
        },
        Err(kronos_api_error) => match kronos_api_error{
            KronosApiError::DbErr(err) => {
                HttpResponse::InternalServerError().body(format!("Database failure: {}\n", err))
            }
            KronosApiError::ActixError(err) => {
                HttpResponse::InternalServerError().body(format!("Internal server error: {}\n", err))
            }
            KronosApiError::NotImplemented(msg) => {
                HttpResponse::NotImplemented().body(format!("Not implemented: {}\n", msg))
            }
            KronosApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().body(format!("Bad request to API: {}\n", msg))
            }
            KronosApiError::ExpectedDataNotPresent(msg) => {
                HttpResponse::InternalServerError().body(format!("Expected data not found in database: {}\n", msg))
            }
            KronosApiError::Unknown(msg) => {
                HttpResponse::BadRequest().body(format!("Unknown error: {}\n", msg))
            }
        }
    }
}

pub async fn access_kronos_database() -> Result<DatabaseConnection, DbErr> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    dprintln!("Configuration read successfully.");
    let connection_string = configuration.database.connection_string();
    dprintln!("Connection string: {}", connection_string);
    Database::connect(connection_string).await 
}

// Invariant: Valid requests always have, at a minimum, a unit and an action.
// This method occurs AFTER deserialization is proven successful.
async fn is_request_valid(req: Json<KronosRequest>) -> Result< Json<KronosRequest>, HttpResponse> {
    // Deserialization successful
    dprintln!("api_handler called, request body: {:?}", req);

    // First, check if the action is null
    if req.action.is_none(){
        return Err(HttpResponse::BadRequest().body(format!("Request action field is null.")));
    }
    
    if req.unit.is_none(){
        return Err(HttpResponse::BadRequest().body(format!("Request unit field is null.")));
    }

    return Ok(req);
    
}

impl KronosRequest {
    pub fn new() -> Self {
        Self {
            action: None,
            unit: None,
            order_id: None,
            paragraph_id: None,
            task_id: None,
        }
    }

    pub fn with_action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_unit(mut self, unit: String) -> Self {
        self.unit = Some(unit);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}