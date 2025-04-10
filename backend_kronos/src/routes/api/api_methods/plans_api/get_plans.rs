//! get_plans.rs

// Basic imports
use actix_web::web::Json;
use debug_print::debug_println as dprintln;
use sea_orm::*;

// Network utilities
use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;

//Helper utilities
use crate::routes::api::helper_methods::summarizers::*;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};
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
        .order_by_desc(plan::Column::SerialNumber)
        .all(&db)
        .await
    {
        Ok(plan_vec) => plan_vec,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    let mut plan_summary_vec: Vec<PlanSummary> = Vec::new();
    // For each plan returned, get it's associated orders
    for plan in plan_vec {
        let packed_plan_summary = match pack_plan_summary_shallow(plan, &db).await {
            Ok(packed_plan_summary) => packed_plan_summary,
            Err(err) => return Err(err),
        };
        plan_summary_vec.push(packed_plan_summary);
    }

    let mut kronos_response = KronosResponse::new(req);

    // Encode them into a KronosResponse Object
    kronos_response.plans_vec = Some(plan_summary_vec);

    // Send back to the client
    Ok(kronos_response)
}
