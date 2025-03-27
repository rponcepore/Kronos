//! paragraph_summary.rs

use crate::models::entities::paragraph::Model as Paragraph;

pub struct ParagraphSummary {
    data: Paragraph,
    subparagraphs: Option<vec<ParagraphSummary>>,
}