//! KronosRequest.tsx
import { KronosApiMethod } from "./KronosApiMethodEnums";
import { PlanRequest } from "./PlanRequest";
import { OrderRequest } from "./OrderRequest";
import { ParagraphRequest } from "./ParagraphRequest";
import { TaskRequest } from "./TaskRequest";

// This file defines a request sent to the backend API.
export type KronosRequest = {
    api_method: KronosApiMethod;
    uic: string;
    // Note: new request subtypes.
    plan_request: PlanRequest | null;
    order_request: OrderRequest | null;
    paragraph_request: ParagraphRequest | null;
    task_request: TaskRequest | null;
}

