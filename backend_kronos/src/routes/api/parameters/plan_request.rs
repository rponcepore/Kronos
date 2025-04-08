//! plan_request.rs
//!
use serde::{Deserialize, Serialize};

// Contained in a KronosRequest
// For create, edit, delete, and get ops

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct PlanRequest {
    pub action: Option<String>,
    // pub uic: Option<String>, // In the future, this might be
    // useful if a user in one unit wanted to create a plan for
    // a user in another unit. For now, a unnecessary edge case.
    pub plan_id: Option<i32>,
    pub plan_name: Option<String>,
}
