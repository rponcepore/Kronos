//! unit_summary.rs

use crate::models::entities::unit::Model as Unit;

use super::plan_summary::PlanSummary;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnitSummary {
    pub data: Unit,
    pub plans: Option<Vec<PlanSummary>>,
}