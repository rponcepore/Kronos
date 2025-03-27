//! unit_summary.rs

use crate::models::entities::unit::Model as Unit;

use super::plan_summary::PlanSummary;

pub struct UnitSummary {
    data: Unit,
    plans: Option<vec<PlanSummary>>,
}