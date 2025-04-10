//! KronosResponse.tsx
import { KronosRequest } from "../networking_types/KronosRequest"
import { PlanSummary } from "../frontend_types/PlanSummary";
import { KronosOrderSummary } from "../frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../frontend_types/ParagraphSummary";
import { UnitSummary } from "../frontend_types/UnitSummary";
import { TaskSummary } from "../frontend_types/TaskSummary";
import { AdminSummary } from "../frontend_types/AdminSummary";

// This defines the type we expect for all kronos API call returns

export type KronosResponse = {
    kronos_request: KronosRequest;
    admin_response: AdminSummary | null;
    units_vec: UnitSummary[] | null;
    plans_vec: PlanSummary[] | null;
    orders_vec: KronosOrderSummary[] | null;
    paragraphs_vec: ParagraphSummary[] | null;
    tasks_vec: TaskSummary[] | null;
}
