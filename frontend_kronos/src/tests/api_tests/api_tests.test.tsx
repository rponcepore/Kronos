import { expect, test } from 'vitest'
import { KronosRequest, KronosRequestBuilder } from '../../types/networking_types/KronosRequest.tsx'
import { PlanSummary } from '../../types/frontend_types/PlanSummary.tsx'
import { kronosApiCall } from '../../helper_methods/ApiCall.tsx'
import { KronosResponse } from '../../types/networking_types/KronosResponse.tsx'
import { KronosApiMethod } from '../../types/networking_types/KronosApiMethodEnums.tsx'
import { KronosOrderSummary } from '../../types/frontend_types/KronosOrderSummary.tsx'
import { PlanRequest } from '../../types/networking_types/PlanRequest.tsx'
import { OrderRequest } from '../../types/networking_types/OrderRequest.tsx'


import { ParagraphSummary } from '../../types/frontend_types/ParagraphSummary.tsx'
import { Classification } from '../../types/enums/Classification.tsx'



  
test('Verbose network test: Verify structure of KronosResponse', async () => {
  const req: KronosRequest = new KronosRequestBuilder ()
    .setUic("tstUIC")
    .setApiMethod(KronosApiMethod.get_plans)
    .build();

  let response: KronosResponse = await kronosApiCall(req);

  expect(response.plans_vec).not.toBeNull();

  // If plans_vec is not null, assert each plan is of type PlanSummary and print its data
  if (response.plans_vec) {
    response.plans_vec.forEach((plan: PlanSummary) => {
      // Assert that each element is a PlanSummary
      expect(plan).toBeInstanceOf(Object); // This checks that the plan is an object (you can refine the check if needed)

      // Print the contents of PlanSummary.data
      //console.log("PlanSummary data:", plan.data);
    });
  }
})

test('Verify Order Template exists in database.', async () => {
  const plan_request: PlanRequest = {
    plan_id: 0,
    plan_name: null,
    classification: null,
  };

  const req: KronosRequest = new KronosRequestBuilder ()
    .setUic("TEMPLT")
    .setApiMethod(KronosApiMethod.get_plans)
    .setPlanRequest(plan_request)
    .build();

  let response: KronosResponse = await kronosApiCall(req);

  expect(response.plans_vec).not.toBeNull();

  // If plans_vec is not null, assert each plan is of type PlanSummary and print its data
  if (response.plans_vec) {
    response.plans_vec.forEach((plan: PlanSummary) => {
      // Assert that each element is a PlanSummary
      expect(plan).toBeInstanceOf(Object); // This checks that the plan is an object (you can refine the check if needed)

      // Print the contents of PlanSummary.data
      //console.log("PlanSummary data:", plan.data);
    });
  }
})

test('Look at a FRAGORD', async () => {
  const plan_request: PlanRequest = {
    plan_id: 0,
    plan_name: null,
    classification: null,
  };

  const req: KronosRequest = {
    api_method: KronosApiMethod.get_plans,
    uic: "WJH8AA",
    plan_request: plan_request,
    order_request: null,
    paragraph_request: null,
    task_request: null,
    admin_request: null,
    unit_request: null,
  };
  let response: KronosResponse = await kronosApiCall(req);

  expect(response.plans_vec).not.toBeNull();

  // If plans_vec is not null, assert each plan is of type PlanSummary and print its data
  if (response.plans_vec) {
    response.plans_vec.forEach((plan: PlanSummary) => {
      // Assert that each element is a PlanSummary
      expect(plan).toBeInstanceOf(Object); // This checks that the plan is an object (you can refine the check if needed)

      // Print the contents of PlanSummary.data
      //console.log("PlanSummary data:", plan.data);
    });
  }
})

test('get_orders endpoint test', async () => {
  const order_request: OrderRequest = {
    order_id: 6,
    parent_plan_id: null,
    order_type: null,
  };


  const kronos_request : KronosRequest = new KronosRequestBuilder ()
    .setUic("WJH8AA")
    .setApiMethod(KronosApiMethod.get_order)
    .setOrderRequest(order_request)
    .build()

  let response: KronosResponse = await kronosApiCall(kronos_request);
  expect(response.orders_vec); // This should not be null, and ~should~ print to the console.
  expect(response.orders_vec).not.toBeNull();
  expect(response.orders_vec?.length).toBeGreaterThan(0); // Ensure it contains at least one order

  const order: KronosOrderSummary = response.orders_vec![0]; // Use '!' since we've checked it above
  //console.dir(order, { depth: null });

})