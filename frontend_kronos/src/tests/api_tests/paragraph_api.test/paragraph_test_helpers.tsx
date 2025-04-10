//! paragraph_test_helpers.tsx

import { assert, expect } from "vitest";
import { ParagraphSummary } from "../../../types/frontend_types/ParagraphSummary";
import { KronosApiMethod } from "../../../types/networking_types/KronosApiMethodEnums";
import { KronosRequest } from "../../../types/networking_types/KronosRequest";
import { KronosOrderSummary } from "../../../types/frontend_types/KronosOrderSummary";
import { kronosApiCall } from "../../../helper_methods/ApiCall";
import { KronosResponse } from "../../../types/networking_types/KronosResponse";
import { ParagraphRequest } from "../../../types/networking_types/ParagraphRequest";

export function checkSubparagraphsAreInOrder(target: ParagraphSummary) : boolean {
    
    // If target has no subparagraphs, nothing to check.
    if (target.subparagraphs === null ) {
        return true;
    } else if (target.subparagraphs.length === 0 ) {
        return true;
    }

    // The target has subparagraphs. 
    for (let i = 0; i < target.subparagraphs.length; i++) {
        const expected = i + 1;
        const actual = target.subparagraphs[i].data.ordinal_sequence;
        if (actual !== expected) {
            console.error(`Expected ordinal_sequence ${expected}, got ${actual}`);
            return false;
        }
    }

    return true;
}





export function extractParagraphFromOrderSummary(paragraphNumber : number,  order : KronosOrderSummary ) : ParagraphSummary {
    expect(order.paragraphs).not.toBeNull();
    expect(order.paragraphs.length).toBeGreaterThan(0);
    expect(order.paragraphs.length).toBeGreaterThanOrEqual(paragraphNumber -1 );
    const paragraph = order.paragraphs[paragraphNumber-1];
    return paragraph;
}

export async function insertParagraphBelowTarget( target : ParagraphSummary, title: string, text: string ) : Promise<KronosResponse> {
    const paragraphRequest : ParagraphRequest = {
        paragraph_id: target.data.id,
        new_title: title,
        new_text: text,
        insert_method: "BELOW",
    };
    const kronosRequest : KronosRequest = {
        api_method: KronosApiMethod.insert_paragraph,
        uic: "WJH8AA",
        plan_request: null,
        order_request: null,
        paragraph_request: paragraphRequest,
        task_request: null,
        admin_request: null,
        unit_request: null,
    };
    const response = await kronosApiCall(kronosRequest);
    return response;
}

export async function insertParagraphAboveTarget( target : ParagraphSummary, title: string, text: string ) : Promise<KronosResponse> {
    const paragraphRequest : ParagraphRequest = {
        paragraph_id: target.data.id,
        new_title: title,
        new_text: text,
        insert_method: "ABOVE",
    };
    const kronosRequest : KronosRequest = {
        api_method: KronosApiMethod.insert_paragraph,
        uic: "WJH8AA",
        plan_request: null,
        order_request: null,
        paragraph_request: paragraphRequest,
        task_request: null,
        admin_request: null,
        unit_request: null,
    };
    const response = await kronosApiCall(kronosRequest);
    return response;
}

export function doesTargetParagraphHaveChildren( target : ParagraphSummary ) : boolean {
    if (target.subparagraphs === null ){
        return false;
    }else if (target.subparagraphs === undefined ) {
        return false;
    }else if (target.subparagraphs.length === 0 ) {
        return false;
    }
    return true;
}

export async function insertSubparagraph( target : ParagraphSummary, title : string, text : string ) : Promise<KronosResponse> {
    const paragraphRequest : ParagraphRequest = {
        paragraph_id: target.data.id,
        new_title: title,
        new_text: text,
        insert_method: "SUBPARAGRAPH",
    };
    const kronosRequest : KronosRequest = {
        api_method: KronosApiMethod.insert_paragraph,
        uic: "WJH8AA",
        plan_request: null,
        order_request: null,
        paragraph_request: paragraphRequest,
        task_request: null,
        admin_request: null,
        unit_request: null,
    };
    const response = await kronosApiCall(kronosRequest);
    return response;
}