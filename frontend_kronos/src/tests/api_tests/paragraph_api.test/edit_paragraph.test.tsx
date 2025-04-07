import { expect, test } from 'vitest'
import { KronosRequest } from '../../../types/networking_types/KronosRequest.tsx'
import { PlanSummary } from '../../../types/frontend_types/PlanSummary.tsx'
import { kronosApiCall } from '../../../helper_methods/ApiCall.tsx'
import { KronosResponse } from '../../../types/networking_types/KronosResponse.tsx'
import { KronosApiMethod } from '../../../types/networking_types/KronosApiMethodEnums.tsx'
import { KronosOrderSummary } from '../../../types/frontend_types/KronosOrderSummary.tsx'
import { PlanRequest } from '../../../types/networking_types/PlanRequest.tsx'
import { OrderRequest } from '../../../types/networking_types/OrderRequest.tsx'
import { ParagraphRequest } from '../../../types/networking_types/ParagraphRequest.tsx'


import { ParagraphSummary } from '../../../types/frontend_types/ParagraphSummary.tsx'
import { Classification } from '../../../types/enums/Classification.tsx'



test('edit_paragraph api test', async () => {
    //first, get an order
    const order_request: OrderRequest = {
        order_id: 6,
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
    };
    let response: KronosResponse = await kronosApiCall(req);
    expect(response.orders_vec); // This should not be null, and ~should~ print to the console.
    expect(response.orders_vec).not.toBeNull();
    expect(response.orders_vec?.length).toBeGreaterThan(0); // Ensure it contains at least one order

    const order: KronosOrderSummary = response.orders_vec![0]; // Use '!' since we've checked it above
    console.dir(order, { depth: null });

    // Now that we have an order, get a paragraph.

    const target = order.paragraphs[0]; // the situation paragraph
    const old_text = target.data.text;
    const old_title = target.data.title;
    const paragraph_request: ParagraphRequest = {
        paragraph_id: target.data.id,
        new_text: "Testing the Situation Paragraph",
        new_title: "Testing the Title of the Situation Paragraph",
        insert_method: null,
    }

    const req2: KronosRequest = {
        api_method : KronosApiMethod.edit_paragraph,
        uic: "WJH8AA",
        plan_request: null,
        order_request: null,
        paragraph_request: paragraph_request,
        task_request: null,
    }
    let response2: KronosResponse = await kronosApiCall(req2);
    expect(response2.paragraphs_vec); //should not be null
    expect(response2.paragraphs_vec?.length).toBe(1); //should only be of size one.
    let return_paragraph_summary : ParagraphSummary | undefined = response2.paragraphs_vec?.[0] ?? undefined; // This is strictly to satisfy the compiler. Undefined behavior would already have been caught.
    expect(return_paragraph_summary?.data.text).toBe("Testing the Situation Paragraph");
    expect(return_paragraph_summary?.data.title).toBe("Testing the Title of the Situation Paragraph");


    // Reset
    const paragraph_request_revert: ParagraphRequest = {
        paragraph_id: target.data.id,
        new_text: old_text,
        new_title: old_title,
        insert_method: null,
    }

    const req2_revert: KronosRequest = {
        api_method : KronosApiMethod.edit_paragraph,
        uic: "WJH8AA",
        plan_request: null,
        order_request: null,
        paragraph_request: paragraph_request_revert,
        task_request: null,
    }
    

    // This should reset all the things
    // Honestly if one of these fails it's entirely my fault. 
    let response2_revert: KronosResponse = await kronosApiCall(req2_revert);
    expect(response2_revert.paragraphs_vec); //should not be null
    expect(response2_revert.paragraphs_vec?.length).toBe(1); //should only be of size one.
    let revert_summary : ParagraphSummary | undefined = response2_revert.paragraphs_vec?.[0] ?? undefined; // This is strictly to satisfy the compiler. Undefined behavior would already have been caught.
    expect(revert_summary?.data.text).toBe(old_text);
    expect(revert_summary?.data.title).toBe(old_title);

})