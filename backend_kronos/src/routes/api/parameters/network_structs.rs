//! network_structs.rs

use serde::{Serialize, Deserialize};

use super::plan_request::PlanRequest;
// For incomign requests.
use crate::models::entities::*;
// For outgoing responses.
use crate::models::entity_summaries::{
    unit_summary::UnitSummary,
    paragraph_summary::ParagraphSummary,
    kronos_order_summary::KronosOrderSummary,
    plan_summary::PlanSummary,
};


#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct KronosRequest {
    //pub request_id: Integer,
    //pub http_method: Option<String>,
    pub api_method: Option<String>,
    pub unit: Option<String>,
    pub order_id: Option<i32>,
    pub paragraph_id: Option<i32>,
    pub task_id: Option<i32>,
    // Basic fields for easy access
    pub plan_request: Option<PlanRequest>,
    
}

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct KronosResponse {
    pub kronos_request: KronosRequest,
    pub plans_vec: Option< Vec< PlanSummary>>,
    pub orders_vec: Option< Vec< KronosOrderSummary>>,
    pub paragraphs_vec: Option< Vec< ParagraphSummary>>,
    pub units_vec: Option< Vec< UnitSummary> >,
}

pub enum KronosApiError  {
    DbErr(sea_orm::DbErr),
    ActixError(actix_web::Error),
    NotImplemented(String),
    BadRequest(String),
    ExpectedDataNotPresent(String),
    Unknown(String),
}