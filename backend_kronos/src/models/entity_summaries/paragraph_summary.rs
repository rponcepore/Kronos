//! paragraph_summary.rs

use crate::models::entities::paragraph::Model as Paragraph;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParagraphSummary {
    pub data: Paragraph,
    pub subparagraphs: Option<Vec<ParagraphSummary>>,
}