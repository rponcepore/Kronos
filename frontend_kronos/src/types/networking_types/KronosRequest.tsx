//! KronosRequest.tsx
import { KronosApiMethod } from "./KronosApiMethodEnums";
import { PlanRequest } from "./PlanRequest";
import { OrderRequest } from "./OrderRequest";
import { ParagraphRequest } from "./ParagraphRequest";
import { TaskRequest } from "./TaskRequest";
import { UnitRequest } from "./UnitRequest";
import { AdminRequest } from "./AdminRequest";

// This file defines a request sent to the backend API.
export type KronosRequest = {
    api_method: KronosApiMethod;
    uic: string; // This is the UIC of the CALLING unit. 
    // Note: new request subtypes.
    admin_request: AdminRequest | null;
    unit_request: UnitRequest | null; // This also has a UIC, but maybe it's because you want some data, or to create a new uic, etc.
    plan_request: PlanRequest | null;
    order_request: OrderRequest | null;
    paragraph_request: ParagraphRequest | null;
    task_request: TaskRequest | null;
}

