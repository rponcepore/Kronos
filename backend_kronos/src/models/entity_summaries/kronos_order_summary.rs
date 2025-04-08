//! kronos_order_summary.rs
use serde::{Deserialize, Serialize};

use super::paragraph_summary::ParagraphSummary;
use crate::models::entities::kronos_order::Model as KronosOrder;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KronosOrderSummary {
    pub data: KronosOrder,
    pub paragraphs: Option<Vec<ParagraphSummary>>,
}
