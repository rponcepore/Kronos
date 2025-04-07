//! OrderRequest.tsx
import { OrderKind } from "../enums/OrderKind"

export type OrderRequest = {
    order_id: number | null,
    parent_plan_id: number | null,
    order_type: OrderKind | null,
}