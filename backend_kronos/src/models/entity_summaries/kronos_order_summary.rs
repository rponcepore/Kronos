//! kronos_order_summary.rs
use serde::{Deserialize, Serialize};

use crate::models::entities::kronos_order::Model as KronosOrder;
use super::paragraph_summary::ParagraphSummary;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KronosOrderSummary {
    pub data: KronosOrder,
    pub paragraphs: Option<Vec<ParagraphSummary>>,
}