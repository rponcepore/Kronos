//! ParagraphRequest.tsx

export type ParagraphRequest = {
    paragraph_id: number | null, // This is the target paragraph
    // parent_paragraph_id: number | null, 
    insert_method: "ABOVE" | "BELOW" | "SUBPARAGRAPH" | null,
    new_title: string | null,
    new_text: string | null,
}