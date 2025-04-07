//! order_request.rs
//! 
use serde::{Serialize, Deserialize};

// Contained in a KronosRequest
// For create, edit, delete, and get ops

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct OrderRequest {
    pub order_id: Option<i32>,
    pub parent_plan_id: Option<i32>,
    pub order_type: Option<String>,
}
