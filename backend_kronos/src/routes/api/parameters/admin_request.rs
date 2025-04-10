//! admin_request.rs

use serde::Serialize;

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct AdminRequest {
    pub admin_action: Option<String>,
}
