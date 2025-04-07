//! task_request.rs

use serde::{Serialize, Deserialize};

// Contained in a KronosRequest
// For create, edit, delete, and get ops

#[derive(serde::Deserialize, Serialize)]
#[derive(Debug)]
pub struct TaskRequest {
    pub task_id: Option<i32>,
}