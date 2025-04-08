//! network_structs.rs

use serde::{Deserialize, Serialize};

use super::order_request::OrderRequest;
use super::paragraph_request::ParagraphRequest;
use super::plan_request::PlanRequest;
use super::task_request::TaskRequest;
// For incomign requests.
use crate::models::entities::*;
// For outgoing responses.
use crate::models::entity_summaries::{
    kronos_order_summary::KronosOrderSummary, paragraph_summary::ParagraphSummary,
    plan_summary::PlanSummary, unit_summary::UnitSummary,
};

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct KronosRequest {
    //pub request_id: Integer,
    pub api_method: Option<String>,
    pub uic: Option<String>,
    // Basic fields for easy access
    pub plan_request: Option<PlanRequest>,
    pub order_request: Option<OrderRequest>,
    pub paragraph_request: Option<ParagraphRequest>,
    pub task_request: Option<TaskRequest>,
}

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct KronosResponse {
    pub kronos_request: KronosRequest,
    pub plans_vec: Option<Vec<PlanSummary>>,
    pub orders_vec: Option<Vec<KronosOrderSummary>>,
    pub paragraphs_vec: Option<Vec<ParagraphSummary>>,
    pub units_vec: Option<Vec<UnitSummary>>,
}

pub enum KronosApiError {
    DbErr(sea_orm::DbErr),
    ActixError(actix_web::Error),
    NotImplemented(String),
    BadRequest(String),
    ExpectedDataNotPresent(String),
    Unknown(String),
}
