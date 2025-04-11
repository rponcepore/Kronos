//! admin_summary.rs
//! 
//! 
use serde::{Serialize, Deserialize};

// Admin is not an entry in the database. All it's for is for answering unstructured questions. 
#[derive(serde::Deserialize, Serialize, Debug)]
pub struct AdminSummary{
    pub number_response: Option<i32>,
    pub string_response: Option<String>,
    pub rows_affected: Option<i32>,
}