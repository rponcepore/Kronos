//! KronosOrderSummary.tsx

import { KronosOrder } from "../backend_types/KronosOrder";
import { ParagraphSummary } from "./ParagraphSummary";

export type KronosOrderSummary = {
    data: KronosOrder;
    paragraphs: ParagraphSummary[];
}