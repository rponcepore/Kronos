//! edit_paragraph.rs
//!

/*
// EDIT PARAGRAPH
-> Allows you to change the text and the title of a paragraph.
KronosRequest: Must include the paragraph text, paragraph title, and paragraph ID (database primary key). Everything else can be inferred.
KronosResponse: Will include the updated ParagraphSummary
Action on Frontend: Up to you. I would just re-render the ParagraphSummary Component.
*/
use actix_web::web::Json;
use debug_print::debug_println as dprintln;
use sea_orm::*;

use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;

use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;

use crate::routes::api::helper_methods::build_paragraph_summary::*;

struct EditParagraphParameters<'a> {
    pub paragraph_id: &'a i32,
    pub new_text: &'a String,
    pub new_title: &'a String,
}

// The request has already been vetted for general compliance, this fn will handle only the special cases.
pub async fn edit_paragraph(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    dprintln!("edit_paragraph called");

    // Check that the reqeust is not null, and has the three necessary fields.
    let checked_params = check_edit_paragraph_request(&req)?;

    // Build the SeaORM query
    let updated_paragraph = paragraph::ActiveModel {
        id: ActiveValue::Set(checked_params.paragraph_id.to_owned()),
        title: ActiveValue::Set(checked_params.new_title.to_owned()),
        text: ActiveValue::Set(checked_params.new_text.to_owned()),
        kronos_order: ActiveValue::NotSet,
        parent_paragraph: ActiveValue::NotSet,
        ordinal_sequence: ActiveValue::NotSet,
        indent_level: ActiveValue::NotSet,
    };

    // Connect to the database
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // Execution
    let result = match updated_paragraph.update(&db).await {
        Ok(result) => result,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // Get the updated model back from the database.
    let paragraph = match Paragraph::find_by_id(result.id).one(&db).await{
        Ok(paragraph) => match paragraph {
            Some(paragraph) => paragraph,
            None => return Err(KronosApiError::ExpectedDataNotPresent("After successful execution of insert query, could not find paragraph. Recommend a full page refresh.".to_string())),
        },
        Err(msg) => return Err(
            KronosApiError::ExpectedDataNotPresent(
                format!("After successful execution of insert query, could not find paragraph. Recommend a full page refresh. DbErr: {}", msg)
                .to_string()
            )
        ),
    };

    /*
    // Now we need the parent paragraph.
    let inserted_paragraph = Paragraph::find_by_id(result.last_insert_id)
        .one(&db)
        .await?;

    let parent_paragraph_id = match inserted_paragraph {
        Some(inserted_paragraph) => match inserted_paragraph.parent_paragraph {
            Some(parent_paragraph_id) => parent_paragraph_id,
            None => panic!("We are crashing. Though we successfully edited the paragraph, the parend paragraph field is now blank. Crash; invariant violated. In a future life (todo) make this graceful."),
        },
        None => return Err(KronosApiError::ExpectedDataNotPresent("After successful execution of insert query, could not find paragraph. Recommend a full page refresh".to_string())),
    };

    let parent_paragraph = Paragraph::find_by_id(parent_paragraph_id).one(&db).await?;

    let parent_paragraph_summary: ParagraphSummary = generate_paragraph_summary().await?;
    */

    let paragraph_summary: ParagraphSummary = match build_paragraph_summary(&paragraph, &db).await {
        Ok(paragraph_summary) => paragraph_summary,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    let response = KronosResponse::new(req).with_paragraph(paragraph_summary);

    Ok(response)
}

fn check_edit_paragraph_request(
    req: &Json<KronosRequest>,
) -> Result<EditParagraphParameters, KronosApiError> {
    let paragraph_request = match &req.paragraph_request {
        Some(paragraph_request) => paragraph_request,
        None => return Err(KronosApiError::BadRequest("edit_paragraph request was submitted with null paragraph_request parameter in the KronosRequest.".to_string())),
    };

    let target_paragraph_id = match &paragraph_request.paragraph_id {
        Some(target_paragraph_id) => target_paragraph_id,
        None => return Err(KronosApiError::BadRequest("No paragaph_id was specified in the edit_paragraph request. Check your paragraph_request parameter in the KronosRequest.".to_string())),
    };

    let target_paragraph_text = match &paragraph_request.new_text {
        Some(target_paragraph_text) => target_paragraph_text,
        None => return Err(KronosApiError::BadRequest("No new_text was specified in the edit_paragraph request. Check your paragraph_request parameter in the KronosRequest.".to_string())),
    };

    let target_paragraph_title = match &paragraph_request.new_title {
        Some(target_paragraph_title) => target_paragraph_title,
        None => return Err(KronosApiError::BadRequest("No new_title was specified in the edit_paragraph request. Check your paragraph_request parameter in the KronosRequest.".to_string())),
    };

    let checked_struct: EditParagraphParameters = EditParagraphParameters {
        paragraph_id: target_paragraph_id,
        new_text: target_paragraph_text,
        new_title: target_paragraph_title,
    };

    Ok(checked_struct)
}
