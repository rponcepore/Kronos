//! admin_request_handler.rs

use actix_web::web::Json;
use sea_orm::*;

use crate::{routes::api::parameters::{admin_request::AdminRequest, network_structs::{
    KronosApiError, KronosRequest, KronosResponse,
}}, utilities::database_tools::access_kronos_database};

use crate::models::entity_summaries::admin_summary::*;
use crate::models::entities::{prelude::*, *};

pub async fn admin_request_handler(
    req: Json<KronosRequest>,
) -> Result<KronosResponse, KronosApiError> {


    // What's the question?
    let valid_action = check_admin_request(&req)?;

    // Send appropriate response
    let admin = match valid_action.as_str() {
        "count_units" => count_units_in_db().await?,
        _ => return Err(KronosApiError::BadRequest(format!("You provided an unknown action: {}", valid_action)))
    };

    let kronos_response = KronosResponse::new(req).with_admin(admin);

    Ok(kronos_response)
}


async fn count_units_in_db() -> Result<AdminSummary, KronosApiError> {

    // start database
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // for now, just send the number of units in the database
    let selection = Unit::find();
    let count_u64 = match selection.count(&db).await {
        Ok(count_u64) => count_u64,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };
    let count_i32: i32 = count_u64.try_into().map_err(|_| {
        KronosApiError::InternalServerError("Too many units in database to fit in i32".to_string())
    })?;

    let admin = AdminSummary {
        number_response: Some(count_i32),
        string_response: None,
    };

    Ok(admin)
}

fn check_admin_request(req: &Json<KronosRequest>) -> Result<String, KronosApiError> {
    let admin_request = match &req.admin_request {
        Some(admin_request) => admin_request,
        None => return Err(KronosApiError::BadRequest("You must provide an admin request for the admin_request action. Mind blowing stuff.".to_string())),
    };
    let action = match &admin_request.admin_action {
        Some(action) => action,
        None => return Err(KronosApiError::BadRequest("You must provide an actino with the admin reqeust for the admin_reqeust api method.".to_string())),
    };
    Ok(action.to_owned())
}