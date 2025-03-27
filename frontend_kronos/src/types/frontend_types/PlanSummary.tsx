// PlanSummary.tsx

import { Plan } from '../backend_types/Plan';
import { KronosOrderSummary } from './KronosOrderSummary';

/*
 * This type represents the easy-access frontend type. It is meant to naturally work with the 
 * backend types, which are strict representations of the database. The transformations are done on the 
 * backend. When sending data, it should be sent as a backend type. When receiving data, it will be received as 
 * a frontend type.
 */

export type PlanSummary = {
    data: Plan; // The original record
    orders: KronosOrderSummary[]; // The orders associated with this paragraph
}