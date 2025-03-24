//! api.rs

// This file defines the api for calls from the frontend.
use actix_web::web;
use actix_web::{
    web::Json,
    http::{header::ContentType, StatusCode}, 
    HttpRequest, HttpResponse, Responder
    };
    
use sea_orm::*;
use serde::{Deserialize, Serialize};
use debug_print::debug_println as dprintln;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};
// Include our database configs
use crate::configuration::get_configuration;

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct KronosRequest {
    //pub request_id: Integer,
    pub http_method: Option<String>,
    pub action: Option<String>,
    pub unit: Option<String>,
}

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct KronosResponse {
    pub kronos_request: KronosRequest,
    pub plans_vec: Option< Vec<plan::Model>>,
    pub orders_vec: Option< Vec<order::Model>>,
    pub paragraphs_vec: Option< Vec<paragraph::Model>>,
    pub units_vec: Option< Vec<unit::Model>>,
}

pub enum KronosApiError  {
    DbErr(sea_orm::DbErr),
    ActixError(actix_web::Error),
    NotImplemented(String),
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

    let req = match kronos_request_as_json {
        Ok(req) => req,
        Err(msg) => return HttpResponse::BadRequest().body(format!("Invalid JSON: {}\n", msg)),
    };

    // Deserialization successful
    dprintln!("api_handler called, request body: {:?}", req);
    let action = match &req.action {
        Some(action) => action.as_str(),
        None => return HttpResponse::BadRequest().body(format!("Request action field is null.")),
    };

    match &req.unit {
        Some(unit) => unit,
        None => return HttpResponse::BadRequest().body(format!("Invalid request: No unit provided")),
    };

    let kronos_response: Result<KronosResponse, KronosApiError> = match action {
        "get_plans" => get_plans(req).await,
        "get_orders" => Err(KronosApiError::NotImplemented("get_orders not implemented.".to_string())),
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
                // Handle database errors
                HttpResponse::InternalServerError().body(format!("Database failure: {}\n", err))
            }
            KronosApiError::ActixError(err) => {
                // Handle Actix errors
                HttpResponse::InternalServerError().body(format!("Internal server error: {}\n", err))
            }
            KronosApiError::NotImplemented(msg) => {
                // Handle unimplemented features
                HttpResponse::NotImplemented().body(format!("Not implemented: {}\n", msg))
            }
            KronosApiError::Unknown(msg) => {
                // Handle unknown errors
                HttpResponse::BadRequest().body(format!("Unknown error: {}\n", msg))
            }
        }
    }
}

pub async fn get_plans(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError>  {
    dprintln!("get_plans method called. ");

    // TODO: Delete this bad, very bad, hack, that is used only for development:
    if req.unit.as_deref().unwrap_or("") == "tstUIC" { // This is a same unwrap because unit was already checked for None
        let plan_vec = vec![
            plan::Model {
                id: 1,
                unit: "WJH8C0".to_string(),
                parent_plan: None,
                fiscal_year: 2025,
                serial_number: 1,
                classification: "Top Secret".to_string(),
                name: "Operation Blackbeard".to_string(),
            },
            plan::Model {
                id: 2,
                unit: "WJH8C0".to_string(),
                parent_plan: None,
                fiscal_year: 2025,
                serial_number: 2,
                classification: "CUI".to_string(),
                name: "Revenge Strategy".to_string(),
            },
            plan::Model {
                id: 3,
                unit: "WJH8C0".to_string(),
                parent_plan: None,
                fiscal_year: 2025,
                serial_number: 3,
                classification: "Secret".to_string(),
                name: "Jack Sparrow's Gambit".to_string(),
            },
        ];
        let kronos_response = KronosResponse {
            kronos_request: req.into_inner(),
            plans_vec: Some(plan_vec),
            orders_vec: None,
            paragraphs_vec: None,
            units_vec: None,
        };
        return Ok(kronos_response);
    } //END of teh very bad hack that should be deleted. (I need to install mocking, oof.)

    // NORMAL execution:
    // Connect to the database
    let db = match access_kronos_database().await{
        Ok(db) => db,
        Err(error) => return Err(KronosApiError::DbErr(error)),
    };

    let unit_str = match &req.unit {
        Some(unit) => unit.as_str(),
        None => return Err(KronosApiError::Unknown("Deserialization error: unit string failure.".to_string())),
    };

    // Get all the plans for that unit
    let plan_vec: Vec<plan::Model> = match Plan::find()
        .filter(plan::Column::Unit.contains(unit_str))
        .order_by_asc(plan::Column::FiscalYear)
        .order_by_asc(plan::Column::SerialNumber)
        .all(&db)
        .await {
            Ok(plan_vec) => plan_vec,
            Err(msg) => return Err(KronosApiError::DbErr(msg)),
        };
    // Unwrap json<KronosRequest> into just a KronosRequest to avoid de-re-de-se-re-serialization issues. 
    let plain_kronos_request = req.into_inner();
    // Encode them into a KronosResponse Object
    let kronos_response = KronosResponse {
        kronos_request: plain_kronos_request,
        plans_vec: Some(plan_vec),
        orders_vec: None,
        paragraphs_vec: None,
        units_vec: None,
    };
    // Send back to the client
    Ok(kronos_response)
}

pub async fn access_kronos_database() -> Result<DatabaseConnection, DbErr> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    dprintln!("Configuration read successfully.");
    let connection_string = configuration.database.connection_string();
    dprintln!("Connection string: {}", connection_string);
    Database::connect(connection_string).await 
}