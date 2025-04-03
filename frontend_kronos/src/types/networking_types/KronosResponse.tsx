//! KronosResponse.tsx
import { KronosRequest  } from "../networking_types/KronosRequest"
import { Plan } from "../backend_types/Plan";
import { KronosOrder } from "../backend_types/KronosOrder";
import { Paragraph } from "../backend_types/Paragraph";
import { Unit } from "../backend_types/Unit";

import { PlanSummary } from "../frontend_types/PlanSummary";
import { KronosOrderSummary } from "../frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../frontend_types/ParagraphSummary";
import { UnitSummary } from "../frontend_types/UnitSummary";

// This defines the type we expect for all kronos API call returns

export type KronosResponse = {
    kronos_request: KronosRequest;
    plans_vec: PlanSummary[] | null;
    orders_vec: KronosOrderSummary[] | null;
    paragraphs_vec: ParagraphSummary[] | null;
    units_vec: UnitSummary[] | null;
    //
}
