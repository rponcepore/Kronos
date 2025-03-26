//! get_plans.rs
//! 

use actix_web::web;
use actix_web::{
    web::Json,
    http::{header::ContentType, StatusCode}, 
    HttpRequest, HttpResponse, Responder
    };
    
use sea_orm::*;
use serde::{Deserialize, Serialize};
use debug_print::debug_println as dprintln;
use crate::routes::api::api_handler::KronosApiError;
use crate::routes::api::api_handler::KronosRequest;
use crate::routes::api::api_handler::KronosResponse;
use crate::routes::api::api_handler::access_kronos_database;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};
// Include our database configs
use crate::configuration::get_configuration;


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
    
    let mut order_vec: Vec<kronos_order::Model> = Vec::new();

    // For each plan returned, get it's associated orders
    for plan in &plan_vec {
        let mut order_vec_single_plan = match KronosOrder::find()
            .filter(kronos_order::Column::ParentPlan.eq(plan.id))
            .order_by_asc(kronos_order::Column::SerialNumber)
            .all(&db)
            .await {
                Ok(order_vec_single_plan) => order_vec_single_plan,
                Err(msg) => return Err(KronosApiError::DbErr(msg)),
            };
        for order in order_vec_single_plan{
            order_vec.push(order);
        }
    }

    // Unwrap json<KronosRequest> into just a KronosRequest to avoid de-re-de-se-re-serialization issues. 
    let plain_kronos_request = req.into_inner();
    // Encode them into a KronosResponse Object
    let kronos_response = KronosResponse {
        kronos_request: plain_kronos_request,
        plans_vec: Some(plan_vec),
        orders_vec: Some(order_vec),
        paragraphs_vec: None,
        units_vec: None,
    };
    // Send back to the client
    Ok(kronos_response)
}
