//! PlanRequest.tsx

/*
 * Use this reequst to parameterize plan requests.
 * As we get more detailed, this will become more 
 * and more necessary to allow the project to expand 
 * and express more complex operations.
 */

import { Classification } from "../enums/Classification"

export type PlanRequest = {
    // action: PlanAction | null,
    plan_id: number | null,
    plan_name: string | null,
    classification: Classification | null,
}
/*
export enum PlanAction {
    create = "create_plan",
    edit = "edit_plan",
    delete = "delete_plan",
    // get = "get_plan",
}
*/