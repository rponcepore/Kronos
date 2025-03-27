//! kronos_order_summary.rs

use crate::models::entities::plan::Model as Plan;

use super::kronos_order_summary::KronosOrderSummary;

pub struct PlanSummary {
    data: Plan,
    orders: Option<vec<KronosOrderSummary>>,
}