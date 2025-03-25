//! KronosResponse.tsx
import { KronosRequest  } from "./KronosRequest"
import { Plan } from "./Plan";
import { Order } from "./Order";
import { Paragraph } from "./Paragraph";
import { Unit } from "./Unit";

// This defines the type we expect for all kronos API call returns

export type KronosResponse = {
    kronos_request: KronosRequest;
    plans_vec: Plan[] | null;
    orders_vec: Order[] | null;
    paragraphs_vec: Paragraph[] | null;
    units_vec: Unit[] | null;
}
