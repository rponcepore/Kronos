//! create_order.rs

// This file defines the internal actions for creating a new order and populating it to the database.

use actix_web::web::Json;
use debug_print::debug_println as dprintln;
use sea_orm::*;

use crate::models::entity_summaries::kronos_order_summary::KronosOrderSummary;
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;
use crate::models::entity_summaries::plan_summary::PlanSummary;
use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;



pub async fn create_order(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    todo!()
}