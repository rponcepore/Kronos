//! OrderRequest.tsx
import { OrderKind } from "../enums/OrderKind"

export type OrderRequest = {
    // action: PlanAction | null,
    parent_plan_id: number | null,
    order_id: number | null,
    order_type: OrderKind | null,
    // string | null,
}