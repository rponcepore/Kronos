//! unit_request.rs

use serde::Serialize;

use crate::reference::army_echelons_enum::Echelon;

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct UnitRequest {
    pub uic: Option<String>,
    pub parent_uic: Option<String>,
    pub echelon: Option<Echelon>,
    pub nickname: Option<String>,
    pub display_name: Option<String>,
    pub short_name: Option<String>,
    pub component: Option<String>,
    pub state_abbrev: Option<String>,
    pub level: Option<i32>,
    pub service_member_capacity: Option<i32>,
}
