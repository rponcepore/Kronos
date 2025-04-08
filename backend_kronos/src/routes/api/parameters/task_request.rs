//! task_request.rs

use serde::Serialize;

// Contained in a KronosRequest
// For create, edit, delete, and get ops

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct TaskRequest {
    pub task_id: Option<i32>,
}
