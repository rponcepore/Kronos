//! kronos_order_summary.rs

use crate::models::entities::kronos_order::Model as KronosOrder;
use super::paragraph_summary::ParagraphSummary;

pub struct KronosOrderSummary {
    data: KronosOrder,
    paragraphs: Option<vec<ParagraphSummary>>,
}