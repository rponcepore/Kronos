//! delete_test_order.tsx

import { kronosApiCall } from "../../helper_methods/ApiCall";
import { KronosOrderSummary } from "../../types/frontend_types/KronosOrderSummary";
import { KronosApiMethod } from "../../types/networking_types/KronosApiMethodEnums";
import { KronosRequest, KronosRequestBuilder } from "../../types/networking_types/KronosRequest";
import { KronosResponse } from "../../types/networking_types/KronosResponse";
import { OrderRequest } from "../../types/networking_types/OrderRequest";
import { UnitRequest, UnitRequestBuilder } from "../../types/networking_types/UnitRequest";

export async function deleteTestOrder(order : KronosOrderSummary) : Promise<KronosResponse> {
    const order_request : OrderRequest = {
        order_id : order.data.id,
        parent_plan_id: null,
        order_type: null,
    }
    const kronos_request : KronosRequest = {
            api_method: KronosApiMethod.delete_order,
            uic: "IGNORE",
            admin_request: null,
            unit_request: null,
            plan_request: null,
            order_request: order_request,
            paragraph_request: null,
            task_request: null,
        }
    return await kronosApiCall(kronos_request);
}

export async function deleteTestOrderComplete(test_uic: string) : Promise<KronosResponse> {
    const unit_request : UnitRequest = new UnitRequestBuilder()
        .setUic(test_uic)
        .build();
    const kronos_request : KronosRequest = new KronosRequestBuilder()
        .setApiMethod(KronosApiMethod.delete_unit)
        .setUic("IGNORE") // for testing only
        .setUnitRequest(unit_request)
        .build();
    let response = await kronosApiCall(kronos_request);
    return response;
}