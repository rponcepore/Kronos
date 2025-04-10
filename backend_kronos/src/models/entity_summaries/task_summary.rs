//! task_summary

//use crate::models::entities::task::Model as Task;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskSummary {
    pub data: Option<String>,
}
