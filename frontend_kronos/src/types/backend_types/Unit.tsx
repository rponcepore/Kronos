//! Unit.tsx

export type Unit = {
    uic: string;
    echelon: string,
    nickname: string,
    display_name: string,
    short_name: string,
    component: string,
    state_abbrev: string,
    level: number,
    service_member_capacity: number | null,
    parent_uic: string | null,
}

/*
pub uic: String,
pub echelon: String,
pub nickname: String,
pub display_name: String,
pub short_name: String,
pub component: String,
pub state_abbrev: String,
pub level: i32,
pub service_member_capacity: i32,
pub parent_uic: String,
*/