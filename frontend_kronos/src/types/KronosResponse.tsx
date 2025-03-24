//! KronosResponse.tsx
import { KronosRequest  } from "./KronosRequest"
import { Plan } from "./planTypes";
import { Order } from "./Order";
import { Paragraph } from "./Paragraph";
import { Unit } from "./Unit";

// This defines the type we expect for al kronos API call returns

export type KronosResponse = {
    kronos_request: KronosRequest;
    plans_vec: Plan[];
    orders_vec: Order[];
    paragraphs_vec: Paragraph[];
    units_vec: Unit[];
}
