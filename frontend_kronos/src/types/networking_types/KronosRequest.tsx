//! KronosRequest.tsx
import { KronosAction } from "./NetworkEnums";
// This file defines a request sent to the backend API.
export type KronosRequest = {
    action: KronosAction;
    unit: string;
    plan_id: number | null;
    order_id: number | null;
    paragraph_id: number | null;
    task_id: number | null;
}
