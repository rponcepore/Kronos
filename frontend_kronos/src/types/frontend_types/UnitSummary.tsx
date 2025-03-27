//! UnitSummary.tsx

import { Unit } from "../backend_types/Unit"
import { PlanSummary } from "./PlanSummary";

export type UnitSummary = {
    data: Unit;
    plans: PlanSummary;
}