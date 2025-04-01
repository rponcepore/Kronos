//! Paragraph.tsx

export type Paragraph = {
    id: number;
    order: number; //the order from which this paragraph descends
    parent_paragraph: number | null; //self join to show parent paragraph
    is_major: boolean;
    ordinal_sequence: number;
    title: string;
    text: string;
    indent_level: number;
}
