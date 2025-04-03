// PlanSummary.tsx

import { assert } from 'vitest';
import { Plan } from '../backend_types/Plan';
import { OrderKind } from '../enums/OrderKind';
import { KronosOrderSummary } from './KronosOrderSummary';
import { ParagraphSummary } from './ParagraphSummary';

/*
 * This type represents the easy-access frontend type. It is meant to naturally work with the 
 * backend types, which are strict representations of the database. The transformations are done on the 
 * backend. When sending data, it should be sent as a backend type. When receiving data, it will be received as 
 * a frontend type.
 */

export type PlanSummary = {
    data: Plan; // The original record
    orders: KronosOrderSummary[]; // The orders associated with this paragraph
    most_recent_mission: String | null;
}

export function getMostRecentOrder(plan: PlanSummary): KronosOrderSummary | null {
    const warnords: KronosOrderSummary[] = [];
    const opords: KronosOrderSummary[] = [];
    const fragords: KronosOrderSummary[] = [];
    for (const orderSummary of plan.orders){
        const order = orderSummary.data;
        switch (order.order_type) {
            case OrderKind.WARNORD:
                warnords.push(orderSummary);
                break;
            case OrderKind.OPORD:
                opords.push(orderSummary);
                break;
            case OrderKind.FRAGORD:
                fragords.push(orderSummary);
                break;
            default:
                //do nothing.
        }
    }

    if (fragords.length > 0) {
        // return largest fragord
        const orderSummaryArray= fragords;
        return orderSummaryArray.reduce((max, item) => (item.data.serial_number > max.data.serial_number ? item : max), orderSummaryArray[0])
    } else {
        if (opords.length > 0) {
            //invariant: There is only ever one opord.
            assert(opords.length === 1, `There is more than one opord! num opords: ${opords.length}`);
            // return the opord
            return opords[0]; 
        }else{
            if (warnords.length > 0) {
                // return the largest warnord
                const orderSummaryArray= warnords;
                return orderSummaryArray.reduce((max, item) => (item.data.serial_number > max.data.serial_number ? item : max), orderSummaryArray[0])
            }else{
                return null; //empty plan
            }
        }
    }
};
