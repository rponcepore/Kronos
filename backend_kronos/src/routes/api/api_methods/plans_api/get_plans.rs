//! get_plans.rs

// Basic imports
use actix_web::web::Json;
use debug_print::debug_println as dprintln;
use sea_orm::*;

// Network utilities
use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;

//Helper utilities
use crate::routes::api::helper_methods::summarizers::pack_plan_summary;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::kronos_order_summary::KronosOrderSummary;
use crate::models::entity_summaries::plan_summary::PlanSummary;

pub async fn get_plans(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    dprintln!("get_plans method called. ");

    // Connect to the database
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(error) => return Err(KronosApiError::DbErr(error)),
    };

    let unit_str = match &req.uic {
        Some(uic) => uic.as_str(),
        None => {
            return Err(KronosApiError::Unknown(
                "Deserialization error: uic string failure.".to_string(),
            ))
        }
    };

    // Get all the plans for that unit
    let plan_vec: Vec<plan::Model> = match Plan::find()
        .filter(plan::Column::Uic.contains(unit_str))
        .order_by_asc(plan::Column::FiscalYear)
        .order_by_asc(plan::Column::SerialNumber)
        .all(&db)
        .await
    {
        Ok(plan_vec) => plan_vec,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    let mut plan_summary_vec: Vec<PlanSummary> = Vec::new();
    // For each plan returned, get it's associated orders
    for plan in plan_vec {
        let packed_plan_summary = match pack_plan_summary(plan, &db).await {
            Ok(packed_plan_summary) => packed_plan_summary,
            Err(err) => return Err(err),
        };
        plan_summary_vec.push(packed_plan_summary);
    }

    // Unwrap json<KronosRequest> into just a KronosRequest to avoid de-re-de-se-re-serialization issues.
    let plain_kronos_request = req.into_inner();

    // Encode them into a KronosResponse Object
    let kronos_response = KronosResponse {
        kronos_request: plain_kronos_request,
        plans_vec: Some(plan_summary_vec),
        orders_vec: None,
        paragraphs_vec: None,
        units_vec: None,
    };
    // Send back to the client
    Ok(kronos_response)
}
