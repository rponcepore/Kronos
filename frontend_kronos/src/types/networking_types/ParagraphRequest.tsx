//! ParagraphRequest.tsx

export type ParagraphRequest = {
    paragraph_id: number | null, // This is the target paragraph
    // parent_paragraph_id: number | null, 
    insert_method: "ABOVE" | "BELOW" | "SUBPARAGRAPH",
    new_title: string,
    new_text: string,
}