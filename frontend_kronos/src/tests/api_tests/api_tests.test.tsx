import { expect, test } from 'vitest'
import { KronosRequest } from '../../types/networking_types/KronosRequest.tsx'
import { PlanSummary } from '../../types/frontend_types/PlanSummary.tsx'
import { kronosApiCall } from '../../helper_methods/ApiCall.tsx'
import { KronosResponse } from '../../types/networking_types/KronosResponse.tsx'
import { KronosOrderSummary } from '../../types/frontend_types/KronosOrderSummary.tsx'
import { ParagraphSummary } from '../../types/frontend_types/ParagraphSummary.tsx'


test('Network test: Attempt to connect to backend via "tstUIC":', async () => {
    const req: KronosRequest = {
      action: "get_plans",
      unit: "tstUIC",
      plan_id: null,
      order_id: null,
      paragraph_id: null,
      task_id: null,
    };
    let response: KronosResponse = await kronosApiCall(req);
    expect(response.plans_vec).not.toBeNull();
  })
  
  test('Verbose network test: Verify structure of KronosResponse', async () => {
    const req: KronosRequest = {
      action: "get_plans",
      unit: "tstUIC",
      plan_id: null,
      order_id: null,
      paragraph_id: null,
      task_id: null,
    };
    let response: KronosResponse = await kronosApiCall(req);
  
    expect(response.plans_vec).not.toBeNull();
  
    // If plans_vec is not null, assert each plan is of type PlanSummary and print its data
    if (response.plans_vec) {
      response.plans_vec.forEach((plan: PlanSummary) => {
        // Assert that each element is a PlanSummary
        expect(plan).toBeInstanceOf(Object); // This checks that the plan is an object (you can refine the check if needed)
  
        // Print the contents of PlanSummary.data
        console.log("PlanSummary data:", plan.data);
      });
    }
  })
  
  test('Verify Order Template exists in database.', async () => {
    const req: KronosRequest = {
      action: "get_plans",
      unit: "TEMPLT",
      plan_id: 0,
      order_id: null,
      paragraph_id: null,
      task_id: null,
    };
    let response: KronosResponse = await kronosApiCall(req);
  
    expect(response.plans_vec).not.toBeNull();
  
    // If plans_vec is not null, assert each plan is of type PlanSummary and print its data
    if (response.plans_vec) {
      response.plans_vec.forEach((plan: PlanSummary) => {
        // Assert that each element is a PlanSummary
        expect(plan).toBeInstanceOf(Object); // This checks that the plan is an object (you can refine the check if needed)
  
        // Print the contents of PlanSummary.data
        console.log("PlanSummary data:", plan.data);
      });
    }
  })

  test('Look at a FRAGORD', async () => {
    const req: KronosRequest = {
      action: "get_plans",
      unit: "WJH8AA",
      plan_id: 0,
      order_id: null,
      paragraph_id: null,
      task_id: null,
    };
    let response: KronosResponse = await kronosApiCall(req);
  
    expect(response.plans_vec).not.toBeNull();
  
    // If plans_vec is not null, assert each plan is of type PlanSummary and print its data
    if (response.plans_vec) {
      response.plans_vec.forEach((plan: PlanSummary) => {
        // Assert that each element is a PlanSummary
        expect(plan).toBeInstanceOf(Object); // This checks that the plan is an object (you can refine the check if needed)
  
        // Print the contents of PlanSummary.data
        console.log("PlanSummary data:", plan.data);
      });
    }
  })

  test('get_orders endpoint test', async () => {
    const req: KronosRequest = {
      action: "get_order",
      unit: "WJH8AA",
      plan_id: null, 
      order_id: 6, // this is bad, hardcoded data. I just happen to know that plan 1 is a WJH8AA plan.
      paragraph_id: null,
      task_id: null,
    };
    let response: KronosResponse = await kronosApiCall(req);
    expect(response.orders_vec); // This should not be null, and ~should~ print to the console.
    expect(response.orders_vec).not.toBeNull();
    expect(response.orders_vec?.length).toBeGreaterThan(0); // Ensure it contains at least one order

    const order: KronosOrderSummary = response.orders_vec![0]; // Use '!' since we've checked it above
    console.dir(order, { depth: null });
    //for (const paragraph of order.paragraphs) {
    //  console.log(`${paragraph.data.ordinal_sequence}. ${paragraph.data.title} ${paragraph.data.text}`);
    //}

  })