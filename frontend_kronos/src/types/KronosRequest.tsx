//! KronosRequest.tsx

// This file defines a request sent to the backend API.
export type KronosRequest = {
    action: string;
    unit: string;
    plan_id: number;
    order_id: number;
    paragraph_id: number;
    task_id: number;
}
