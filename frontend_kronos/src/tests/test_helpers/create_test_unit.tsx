//! create_test_unit.tsx

import { expect } from "vitest";
import { kronosApiCall } from "../../helper_methods/ApiCall";
import { Echelon } from "../../types/enums/Echelon";
import { UnitSummary } from "../../types/frontend_types/UnitSummary";
import { KronosApiMethod } from "../../types/networking_types/KronosApiMethodEnums";
import { KronosRequest } from "../../types/networking_types/KronosRequest";
import { UnitRequest } from "../../types/networking_types/UnitRequest";



export async function create_test_unit() : Promise<UnitSummary> {
    const uic = generate_test_uic();
    const unit_request : UnitRequest = {
        uic: uic,
        parent_uic: null,
        echelon: Echelon.BDE,
        nickname: generate_name("nickname", 10),
        display_name: generate_name("display_name", 10),
        short_name: generate_name("short_name", 10),
        component: "ACTIVE",
        state_abbrev: null,
        level: 5,
        service_member_capacity: null,
    }
    const kronos_request : KronosRequest = {
        api_method: KronosApiMethod.create_unit,
        uic: "WJH8AA",
        admin_request: null,
        unit_request: unit_request,
        plan_request: null,
        order_request: null,
        paragraph_request: null,
        task_request: null,
    }

    const response = await kronosApiCall(kronos_request);
    expect(response).not.toBeNull();
    expect(response.units_vec).not.toBeNull();
    expect(response.units_vec!.length).toBe(1); // One and only one entry
    const unit_summary : UnitSummary = response.units_vec![0];
    return unit_summary
}

// This function generates a test UIC, which is a string of no more than 80
// characters, the first five of which are "test_", and the remaining 75
// are random characters in A-Z, a-z, or 0-9.
export function generate_test_uic() : string {
    const prefix = "test_";
    const length = 75;
    const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let result = prefix;

    for (let i = 0; i < length; i++) {
        const randomIndex = Math.floor(Math.random() * characters.length);
        result += characters[randomIndex];
    }

    return result;
}

export function generate_name(prefix: string, length: number) : string {
    const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let result = prefix;

    for (let i = 0; i < length; i++) {
        const randomIndex = Math.floor(Math.random() * characters.length);
        result += characters[randomIndex];
    }

    return result;
}