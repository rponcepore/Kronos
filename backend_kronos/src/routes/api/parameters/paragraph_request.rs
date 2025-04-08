//! paragraph_request.rs

use serde::{Deserialize, Serialize};

// Contained in a KronosRequest
// For create, edit, delete, and get ops

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct ParagraphRequest {
    pub paragraph_id: Option<i32>,
    pub insert_method: Option<String>,
    pub new_title: Option<String>,
    pub new_text: Option<String>,
}
