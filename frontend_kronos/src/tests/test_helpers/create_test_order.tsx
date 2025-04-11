//! create_test_order.tsx

import { expect } from "vitest";
import { kronosApiCall } from "../../helper_methods/ApiCall";
import { OrderKind } from "../../types/enums/OrderKind";
import { KronosApiMethod } from "../../types/networking_types/KronosApiMethodEnums";
import { KronosRequest } from "../../types/networking_types/KronosRequest";
import { OrderRequest } from "../../types/networking_types/OrderRequest";
import { KronosOrderSummary } from "../../types/frontend_types/KronosOrderSummary";

export async function create_test_order(uic : string, plan_id : number) : Promise<KronosOrderSummary> {
    const order_request : OrderRequest = {
        order_id: null,
        parent_plan_id: plan_id,
        order_type: OrderKind.OPORD,
    }
    const kronos_request : KronosRequest = {
        uic : uic,
        api_method: KronosApiMethod.create_order,
        admin_request: null,
        unit_request: null,
        plan_request: null,
        order_request: order_request,
        paragraph_request: null,
        task_request: null,
    }
    let response = await kronosApiCall(kronos_request);

    expect(response).not.toBeNull();
    expect(response.orders_vec).not.toBeNull();
    expect(response.orders_vec!.length).toBe(1); //one and only one order.
    return response.orders_vec![0];
}