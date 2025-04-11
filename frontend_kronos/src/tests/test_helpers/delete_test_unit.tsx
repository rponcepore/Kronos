//! delete_test_unit.tsx

import { expect } from "vitest";
import { kronosApiCall } from "../../helper_methods/ApiCall";
import { Echelon } from "../../types/enums/Echelon";
import { UnitSummary } from "../../types/frontend_types/UnitSummary";
import { KronosApiMethod } from "../../types/networking_types/KronosApiMethodEnums";
import { KronosRequest } from "../../types/networking_types/KronosRequest";
import { UnitRequest, UnitRequestBuilder } from "../../types/networking_types/UnitRequest";
import { AdminSummary } from "../../types/frontend_types/AdminSummary";



export async function delete_test_unit(uic: string) : Promise<AdminSummary> {
    
    const unit_request : UnitRequest = new UnitRequestBuilder()
        .setUic(uic)
        .build();
    

    const kronos_request : KronosRequest = {
        api_method: KronosApiMethod.delete_unit,
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
    expect(response.admin_vec).not.toBeNull();
    expect(response.admin_vec!.length).toBe(1); // One and only one entry
    const admin_summary : AdminSummary = response.admin_vec![0];
    return admin_summary;
}