//! Paragraph.tsx

export type Paragraph = {
    id: number;
    order: number; //the order from which this paragraph descends
    parent_paragraph: number; //self join to show parent paragraph
    is_major: boolean;
    ordinal_sequence: number;
    title: string;
    text: string;
    indent_level: number;
}

/*

    pub id: i32,
    pub order: i32,
    pub parent_paragraph: i32,
    pub is_major: bool,
    pub ordinal_sequence: i32,
    pub title: String,
    pub text: String,
    pub indent_level: i32,

*/