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

export class UnitRequestBuilder {
    private unit: UnitRequest = {
        uic: null,
        parent_uic: null,
        echelon: null,
        nickname: null,
        display_name: null,
        short_name: null,
        component: null,
        state_abbrev: null,
        level: null,
        service_member_capacity: null,
    };

    setUic(uic: string): this {
        this.unit.uic = uic;
        return this;
    }

    setParentUic(parent_uic: string): this {
        this.unit.parent_uic = parent_uic;
        return this;
    }

    setEchelon(echelon: Echelon): this {
        this.unit.echelon = echelon;
        return this;
    }

    setNickname(nickname: string): this {
        this.unit.nickname = nickname;
        return this;
    }

    setDisplayName(displayName: string): this {
        this.unit.display_name = displayName;
        return this;
    }

    setShortName(shortName: string): this {
        this.unit.short_name = shortName;
        return this;
    }

    setComponent(component: string): this {
        this.unit.component = component;
        return this;
    }

    setStateAbbrev(stateAbbrev: string): this {
        this.unit.state_abbrev = stateAbbrev;
        return this;
    }

    setLevel(level: number): this {
        this.unit.level = level;
        return this;
    }

    setServiceMemberCapacity(capacity: number): this {
        this.unit.service_member_capacity = capacity;
        return this;
    }

    build(): UnitRequest {
        return this.unit;
    }
}
