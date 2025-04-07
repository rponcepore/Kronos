//! paragraph_request.rs

use serde::{Serialize, Deserialize};

// Contained in a KronosRequest
// For create, edit, delete, and get ops

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct ParagraphRequest {
    pub paragraph_id: Option<i32>,
}