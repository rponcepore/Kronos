//! UnitRequest.tsx

import { Echelon } from "../enums/Echelon"

export type UnitRequest = {
    uic: string | null,
    parent_uic: string | null,
    echelon: Echelon | null,
    nickname: string | null,
    display_name: string | null,
    short_name: string | null,
    component: string | null,
    state_abbrev: string | null,
    level: number | null,
    service_member_capacity: number | null,
}