//! insert_paragraph.rs

// This file defines the method called for inserting a paragraph into an order. 
// INSERT PARAGRAPH
/* 
-> Inserts a paragraph above or below the target paragraph.
KronosRequest: Must include the paragraph that you are acting on, a parameter (insert_method) that dictates if you're inserting it above or below the target, and the title and text of the incoming paragraph.
KronosResponse: A ParagraphSummary of the *PARENT PARAGRAPH* of the inserted paragraph. This is because on teh backedn I will be renumbering all of the paragraphs that are affected, (i.e., any paragraphs below what you just inserted.) The implication is that if you are editing multiple paragraphs, you could potentially lose edits. This is a feature, not a bug. Once the system scales and we have multiple people nugging on the same order, we can enforce some basic locking at the paragraph level. This means you want people editing one and only one paragraph at a time to avoid bogging down other editors. 
Action-> Rerender the PARENT PARAGRAPH component, which should then recursively render the created paragraph and the below paragraphs will receive correct nubers.

-> You could also insert a subparagraph, i.e., instead of adding a paragraph 1.C. by clicking on paragraph 1.B., you could add paragraph 1.C by adding a subparagraph to 1 or a sibling paragraph to 1.B. Or more easily understood, you could add a "Friendly Forces 1 level up" subparagraph to 1.B.1., which would then be 1.B.1.A. In those cases, you would send a similar request. Send the paragraph you are acting on, and insert_method will be "SUBPARAGRAPH". The KronosResponse will return the target paragraph ParagraphSummary.
*/

use actix_web::web::Json;
use sea_orm::*;
use debug_print::debug_println as dprintln;

use crate::routes::api::parameters::{network_structs::*, paragraph_request};
use crate::utilities::database_tools::access_kronos_database;

use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;

use crate::routes::api::helper_methods::assemble_paragraph_summary::*;

// The request has already been vetted for general compliance, this fn will handle only the special cases.
pub async fn insert_paragraph(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    todo!()
}