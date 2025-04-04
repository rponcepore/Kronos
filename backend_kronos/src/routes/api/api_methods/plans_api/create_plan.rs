//! create_plan.rs

// This file creates a plan in the database based on the create_plan API call

// Basic imports
use actix_web::web::Json;
use sea_orm::*;
use debug_print::debug_println as dprintln;

// Network utilities
use crate::routes::api::parameters::{network_structs::*, *};
use crate::utilities::database_tools::access_kronos_database;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::plan_summary::PlanSummary;

// Plans API needed utilities
use crate::utilities::calendar_math::get_federal_fiscal_year;
use crate::routes::api::api_methods::plans_api::plans_api_utilities::*;


pub async fn create_plan(valid_req: Json<KronosRequest>) ->  Result<KronosResponse, KronosApiError>  {
    // Read out unit name. 
    let uic = match &valid_req.uic {
        Some(uic) =>  uic,
        None => return Err(KronosApiError::Unknown("Deserialization error: unit string failure.".to_string())),
    };

    let plan_name = match &valid_req.plan_request{
        Some(plan_request) => match &plan_request.plan_name {
            Some(plan_name) => plan_name,
            None => return Err(KronosApiError::BadRequest("User did not provide a plan name to initialize plan.".to_string()))
        },
        None => return Err(KronosApiError::BadRequest("User did not provide a plan request to initialize plan.".to_string())),
    };

    // Then, deduce the fiscal year
    let fiscal_year = get_federal_fiscal_year();

    // Then, deduce the serial number (database call)
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };
    let max_current_plan_number: i32 = match get_plan_with_highest_serial(&db, uic, &fiscal_year).await {
        Ok(plan) => match plan {
            Some(max_current_plan) => max_current_plan.serial_number,
            None => 0,
        },
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };
    // Then, create the new plan (database call)

    let new_plan = plan::ActiveModel {
        name: ActiveValue::Set(plan_name.to_owned()),
        fiscal_year: ActiveValue::Set(fiscal_year),
        serial_number: ActiveValue::Set(max_current_plan_number+1),
        uic: ActiveValue::Set(uic.to_owned()),
        classification: ActiveValue::Set("CUI".to_owned()),
        parent_plan: ActiveValue::Set(None),
        ..Default::default()
    };

    let plan_pk = match Plan::insert(new_plan).exec(&db).await {
        Ok(result) => result.last_insert_id,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // get the record
    let plan = match Plan::find()
        .filter(plan::Column::Id.eq(plan_pk))
        .one(&db)
        .await {
            Ok(plan) => plan,
            Err(msg) => return Err(KronosApiError::DbErr(msg)),
        };

    let unwrapped_plan = match plan {
        Some(unwrapped_plan) => unwrapped_plan,
        None => return Err(
            KronosApiError::ExpectedDataNotPresent("Plan just inserted could not be retrieved.".to_string())
        ),
    };

    let plan_summary: PlanSummary = PlanSummary {
        data: unwrapped_plan,
        orders: None, // Don't bother with the lookup; nothing has been created yet.
        most_recent_mission: None, // Don't bother with the lookup; nothing has been created yet.
    };

    // Unwrap json<KronosRequest> into just a KronosRequest to avoid de-re-de-se-re-serialization issues. 
    let plain_kronos_request = valid_req.into_inner();

    // Return a KronosResponse with the confirmed data: plan name, fiscal year, serial number, unit
    let response = KronosResponse{
        kronos_request: plain_kronos_request,
        plans_vec: Some(vec![plan_summary]),
        orders_vec: None,
        paragraphs_vec: None,
        units_vec: None,
    };

    Ok(response)

}

