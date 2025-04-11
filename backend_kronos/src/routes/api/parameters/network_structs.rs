//! network_structs.rs

use actix_web::web::Json;
use serde::Serialize;

use super::{
    admin_request::AdminRequest, order_request::OrderRequest, paragraph_request::ParagraphRequest,
    plan_request::PlanRequest, task_request::TaskRequest, unit_request::UnitRequest,
};

// For incomign requests.
// For outgoing responses.
use crate::models::entity_summaries::{
    kronos_order_summary::KronosOrderSummary, 
    paragraph_summary::ParagraphSummary,
    plan_summary::PlanSummary, 
    task_summary::TaskSummary, 
    unit_summary::UnitSummary,
    admin_summary::AdminSummary,
};

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct KronosRequest {
    //pub request_id: Integer,
    pub api_method: Option<String>,
    pub uic: Option<String>,
    // Basic fields for easy access
    pub admin_request: Option<AdminRequest>,
    pub unit_request: Option<UnitRequest>,
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
    pub tasks_vec: Option<Vec<TaskSummary>>,
    pub admin_vec: Option<Vec<AdminSummary>>,
}

#[derive(Debug)]
pub enum KronosApiError {
    DbErr(sea_orm::DbErr),
    ActixError(actix_web::Error),
    NotImplemented(String),
    BadRequest(String),
    ExpectedDataNotPresent(String),
    InternalServerError(String),
    Unknown(String),
}

impl KronosResponse {
    pub fn new(req: Json<KronosRequest>) -> Self {
        Self {
            kronos_request: req.into_inner(),
            plans_vec: None,
            orders_vec: None,
            paragraphs_vec: None,
            units_vec: None,
            tasks_vec: None,
            admin_vec: None,
        }
    }

    pub fn with_plan(mut self, plan_summary: PlanSummary) -> Self {
        self.plans_vec = Some(vec![plan_summary]);
        self
    }

    pub fn with_order(mut self, order: KronosOrderSummary) -> Self {
        self.orders_vec = Some(vec![order]);
        self
    }

    pub fn with_paragraph(mut self, paragraph_summary: ParagraphSummary) -> Self {
        self.paragraphs_vec = Some(vec![paragraph_summary]);
        self
    }

    pub fn with_unit(mut self, unit: UnitSummary) -> Self {
        self.units_vec = Some(vec![unit]);
        self
    }

    pub fn with_admin(mut self, admin: AdminSummary) -> Self {
        self.admin_vec = Some(vec![admin]);
        self
    }

    pub fn with_task(mut self, task: TaskSummary) -> Self {
        self.tasks_vec = Some(vec![task]);
        self
    }
}

impl KronosRequest {
    pub fn new() -> Self {
        Self {
            api_method: None,
            uic: None,
            admin_request: None,
            unit_request: None,
            plan_request: None,
            order_request: None,
            paragraph_request: None,
            task_request: None,
        }
    }

    pub fn with_action(mut self, api_method: String) -> Self {
        self.api_method = Some(api_method);
        self
    }

    pub fn with_unit(mut self, uic: String) -> Self {
        self.uic = Some(uic);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
