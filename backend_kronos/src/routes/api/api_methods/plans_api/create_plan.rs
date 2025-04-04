//! create_plan.rs

// This file creates a plan in the database based on the create_plan API call

use actix_web::web::Json;
use sea_orm::*;
use debug_print::debug_println as dprintln;

use crate::models::entity_summaries::plan_summary::PlanSummary;
use crate::routes::api::api_handler::{KronosApiError, KronosRequest, KronosResponse};
use crate::utilities::database_tools::access_kronos_database;
use crate::utilities::calendar_math::get_federal_fiscal_year;
use crate::routes::api::api_methods::plans_api::plans_api_utilities::*;

use crate::models::entities::{prelude::*, *};

pub async fn create_plan(valid_req: Json<KronosRequest>) ->  Result<KronosResponse, KronosApiError>  {
    // Read out unit name. 
    let uic = match &req.unit {
        Some(uic) => match uic.as_str(){
            Some(uic) => uic,
            None => return Err(
                KronosApiError::Unknown("Deserialization error: unit string failure. UIC is empty string".to_string())
            ),
        },
        None => return Err(KronosApiError::Unknown("Deserialization error: unit string failure.".to_string())),
    };

    // Then, deduce the fiscal year
    let fiscal_year = get_federal_fiscal_year();

    // Then, deduce the serial number (database call)
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // Then, create the new plan (database call)

}

