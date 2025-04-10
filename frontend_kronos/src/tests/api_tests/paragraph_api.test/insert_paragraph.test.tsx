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



test('insert_subparagraph api test', async () => {
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
    expect(order.paragraphs).not.toBeNull();
    expect(order.paragraphs.length).toBe(5);
    console.log(order.paragraphs);
    const target : ParagraphSummary = order.paragraphs[0]; // the situation paragraph
    expect(target.data.title.toUpperCase()).toBe("SITUATION");

    // How many subparagraphs does it currently have? 
    const targetSubParaArrLen = target.subParagraphs.length;
    const targetId = target.data.id;
    const targetText = target.data.text;
    const targetTitle = target.data.title;
    
    const paragraph_request: ParagraphRequest = {
        paragraph_id: target.data.id,
        new_text: "Testing insert subparagraph for Test of the Situation Paragraph",
        new_title: "Testing insert subparagraph for the Title of the Situation Paragraph",
        insert_method: "SUBPARAGRAPH",
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
    expect(response2.paragraphs_vec).not.toBeNull(); //should not be null
    expect(response2.paragraphs_vec!.length).toBe(1); //should only be of size one. (The parent paragraph)
    // This is strictly to satisfy the compiler. Undefined behavior would already have been caught.
    let return_paragraph_summary : ParagraphSummary = response2.paragraphs_vec![0]; 
    // The subparagraph contract dictates that the parent paragraph is returned. 
    expect(return_paragraph_summary.data.text).toBe(targetText);
    expect(return_paragraph_summary.data.title).toBe(targetTitle);
    expect(return_paragraph_summary.data.id).toBe(targetId);

    // We've checked basics. This assertion actually checks our operation.
    expect(return_paragraph_summary.subParagraphs.length).toBe(targetSubParaArrLen + 1); 
    const newSubParagraph = return_paragraph_summary.subParagraphs![0];
    expect(newSubParagraph.data.text).toBe("Testing insert subparagraph for Test of the Situation Paragraph");
    expect(newSubParagraph.data.title).toBe("Testing insert subparagraph for the Title of the Situation Paragraph");

    // Are all numbers in serial length?

    // We're checking a delete now. 
    const paragraph_request_revert: ParagraphRequest = {
        paragraph_id: newSubParagraph.data.id,
        new_text: null,
        new_title: null,
        insert_method: null,
    }

    const req2_revert: KronosRequest = {
        api_method : KronosApiMethod.delete_paragraph,
        uic: "WJH8AA",
        plan_request: null,
        order_request: null,
        paragraph_request: paragraph_request_revert,
        task_request: null,
    }
    

    // This should reset all the things
    // Honestly if one of these fails it's entirely my fault. 
    // This is sending back the parent paragraph.
    let response2_revert: KronosResponse = await kronosApiCall(req2_revert);
    expect(response2_revert); //should not be null
    expect(response2_revert.paragraphs_vec).not.toBeNull();
    expect(response2_revert.paragraphs_vec!.length).toBe(1); //should only be of size one.
    const newTargetParagraph = response2_revert.paragraphs_vec![0];
    expect(newTargetParagraph.subParagraphs.length).toBe(targetSubParaArrLen);
    expect(newTargetParagraph.data.text).toBe(targetText);
    expect(newTargetParagraph.data.title).toBe(targetTitle);

})