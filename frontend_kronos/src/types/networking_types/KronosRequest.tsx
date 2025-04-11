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

export class KronosRequestBuilder {
    private request: KronosRequest = {
        api_method: "UNDEFINED" as KronosApiMethod, // or use a sensible default
        uic: "",
        admin_request: null,
        unit_request: null,
        plan_request: null,
        order_request: null,
        paragraph_request: null,
        task_request: null,
    };

    setApiMethod(method: KronosApiMethod): this {
        this.request.api_method = method;
        return this;
    }

    setUic(uic: string): this {
        this.request.uic = uic;
        return this;
    }

    setAdminRequest(admin: AdminRequest): this {
        this.request.admin_request = admin;
        return this;
    }

    setUnitRequest(unit: UnitRequest): this {
        this.request.unit_request = unit;
        return this;
    }

    setPlanRequest(plan: PlanRequest): this {
        this.request.plan_request = plan;
        return this;
    }

    setOrderRequest(order: OrderRequest): this {
        this.request.order_request = order;
        return this;
    }

    setParagraphRequest(paragraph: ParagraphRequest): this {
        this.request.paragraph_request = paragraph;
        return this;
    }

    setTaskRequest(task: TaskRequest): this {
        this.request.task_request = task;
        return this;
    }

    build(): KronosRequest {
        return this.request;
    }
}

/* EXAMPLE USAGE
const req = new KronosRequestBuilder()
    .setApiMethod("CREATE_UNIT")
    .setUic("HQ123")
    .setUnitRequest(new UnitRequestBuilder().setUic("NEW123").setLevel(3).build())
    .build();

*/