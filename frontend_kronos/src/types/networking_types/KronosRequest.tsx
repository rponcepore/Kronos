//! KronosRequest.tsx

// This file defines a request sent to the backend API.
export type KronosRequest = {
    action: string;
    unit: string;
    plan_id: number | null;
    order_id: number | null;
    paragraph_id: number | null;
    task_id: number | null;
    indent_level?: number;
    ordinal_sequence?: number;
    parent_paragraph?: number | null;
}
