//! kronos_order_summary.rs

use crate::models::entities::plan::Model as Plan;

use super::kronos_order_summary::KronosOrderSummary;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanSummary {
    pub data: Plan,
    pub orders: Option<Vec<KronosOrderSummary>>,
    pub most_recent_mission: String,
}