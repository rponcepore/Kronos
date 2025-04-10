//! unit_request.rs

use serde::Serialize;

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct UnitRequest {
    pub uic: Option<String>,
}
