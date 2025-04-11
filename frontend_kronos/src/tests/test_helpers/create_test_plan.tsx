//! create_test_plan

import { expect } from "vitest";
import { kronosApiCall } from "../../helper_methods/ApiCall";
import { PlanSummary } from "../../types/frontend_types/PlanSummary";
import { KronosApiMethod } from "../../types/networking_types/KronosApiMethodEnums";
import { KronosRequest } from "../../types/networking_types/KronosRequest";
import { PlanRequest } from "../../types/networking_types/PlanRequest";
import { generate_name } from "./create_test_unit";

export async function create_test_plan(uic : string ) : Promise<PlanSummary> {
    let plan_request : PlanRequest = {
        plan_id: null,
        plan_name: generate_name("test_plan", 10),
        classification: null,
    }
    let kronos_request : KronosRequest = {
        uic: uic,
        api_method: KronosApiMethod.create_plan,
        admin_request: null,
        unit_request: null,
        plan_request: plan_request,
        order_request: null,
        paragraph_request: null,
        task_request: null,
    }
    let response = await kronosApiCall(kronos_request);
    expect(response).not.toBeNull();
    expect(response.plans_vec).not.toBeNull();
    expect(response.plans_vec!.length).toBe(1); // Should return one and only one plan
    let planSummary = response.plans_vec![0];
    return planSummary;
}