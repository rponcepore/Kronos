//! ParagraphSummary.tsx

import { Paragraph } from "../backend_types/Paragraph"

export type ParagraphSummary = {
    data: Paragraph;
    subParagraphs: ParagraphSummary[];
}