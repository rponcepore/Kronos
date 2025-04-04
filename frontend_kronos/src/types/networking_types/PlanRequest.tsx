//! PlanRequest.tsx

/*
 * Use this reequst to parameterize plan requests.
 * As we get more detailed, this will become more 
 * and more necessary to allow the project to expand 
 * and express more complex operations.
 */

import { Classification } from "../enums/Classification"

export type PlanRequest = {
    plan_id: number | null,
    plan_name: string | null,
    classification: Classification | null,
}
