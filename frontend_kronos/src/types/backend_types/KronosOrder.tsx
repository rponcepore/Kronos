//! Order.tsx

import { OrderKind } from "../enums/OrderKind";

// This file defines the order type
/*
pub id: i32,
    pub parent_plan: i32,
    pub order_type: String,
    pub serial_number: i32,
    pub is_published: bool,
    pub derived_from: Option<i32>,
*/

export type KronosOrder = {
    id: number,
    parent_plan: number;
    order_type: OrderKind;
    serial_number: number;
    is_published: boolean;
    derived_from: number | null; // i.e., the order id of the higher echelon order from which this order follows
}