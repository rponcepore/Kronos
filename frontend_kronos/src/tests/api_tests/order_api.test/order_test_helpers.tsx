//! order_test_helpers.tsx
import {expect, test} from 'vitest';
import { KronosApiMethod } from "../../../types/networking_types/KronosApiMethodEnums";
import { KronosRequest } from "../../../types/networking_types/KronosRequest";
import { KronosOrderSummary } from "../../../types/frontend_types/KronosOrderSummary";
import { kronosApiCall } from "../../../helper_methods/ApiCall";
import { KronosResponse } from "../../../types/networking_types/KronosResponse";
import { OrderRequest } from '../../../types/networking_types/OrderRequest';
import { create_test_unit } from '../../test_helpers/create_test_unit';
import { UnitSummary } from '../../../types/frontend_types/UnitSummary';
import { PlanSummary } from '../../../types/frontend_types/PlanSummary';
import { create_test_order } from '../../test_helpers/create_test_order';
import { create_test_plan } from '../../test_helpers/create_test_plan';

export async function getTestOrder() : Promise<{uic: string, order : KronosOrderSummary}> {
    let unit : UnitSummary = await create_test_unit();
    let plan : PlanSummary = await create_test_plan(unit.data.uic);
    let order : KronosOrderSummary = await create_test_order(unit.data.uic, plan.data.id); 
    return {
        uic: unit.data.uic,
        order: order,
    }
}

export async function getOrder(order_id : number) : Promise<KronosOrderSummary> {
    const order_request: OrderRequest = {
            order_id: order_id,
            parent_plan_id: null,
            order_type: null,
        };
        
        const req: KronosRequest = {
            api_method: KronosApiMethod.get_order,
            uic: "WJH8AA",
            plan_request: null,
            order_request: order_request,
            paragraph_request: null,
            task_request: null,
            admin_request: null,
            unit_request: null,
        };

    let response: KronosResponse = await kronosApiCall(req);
    expect(response.orders_vec); // This should not be null, and ~should~ print to the console.
    expect(response.orders_vec).not.toBeNull();
    expect(response.orders_vec!.length).toBeGreaterThan(0); 
    expect(response.orders_vec!.length).toBe(1);

    // Extract the order
    const order : KronosOrderSummary = response.orders_vec![0];
    return order;
}