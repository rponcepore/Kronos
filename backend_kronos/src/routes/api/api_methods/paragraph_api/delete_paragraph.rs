//!delete_paragraph.rs

use actix_web::web::Json;
use debug_print::debug_println as dprintln;
use sea_orm::*;

use crate::routes::api::parameters::{network_structs::*, paragraph_request};
use crate::utilities::database_tools::access_kronos_database;

use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;

use crate::routes::api::helper_methods::assemble_paragraph_summary::*;
use crate::routes::api::api_methods::paragraph_api::paragraph_helper_methods::*;

struct DeleteParagraphParams<'a> {
    paragraph_id: &'a i32,
}

pub async fn delete_paragraph(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    dprintln!("delete_paragraph called");

    // Ensure we have a good request
    let checked_params = check_delete_paragraph_request(&req)?;

    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // Identify the target paragraph
    let target_paragraph_record = match get_target_record(checked_params.paragraph_id, &db).await{
        Ok(target_paragraph_record) => target_paragraph_record,
        Err(msg) => return Err(msg),
    };

    // Get the parent paragraph
    let parent = get_parent_paragraph(&target_paragraph_record, &db).await?;

    let ordinal_start = target_paragraph_record.ordinal_sequence;

    // Define delete query
    let delete_entry = paragraph::ActiveModel {
        id: ActiveValue::Set(target_paragraph_record.id),
        ..Default::default()
    };

    // Execute
    let result = match delete_entry.delete(&db).await {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    // Find affected paragraphs
    let para_vec = match Paragraph::find()
        .filter(paragraph::Column::Id.eq(parent.id))
        .all(&db)
        .await
        {
            Ok(para_vec) => para_vec,
            Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
        };
    
    //Cascade update all the paragraphs below it to add one to their numbers
    for paragraph in para_vec {
        //check if the ordinal sequence is equal to or greater than the ordinal sequence we need to insert
        // if yes, +=1 , else ignore
        if paragraph.ordinal_sequence >= ordinal_start {
            // Ensure we send this to the database!
            adjust_ordinal_sequence(-1, paragraph, &db).await?;
        }
    }

    // Return the updated parent paragraph
    let updated_parent = get_parent_paragraph(&target_paragraph_record, &db).await?;
    let paragraph_summary = match assemble_paragraph_summary(&updated_parent, &db).await {
        Ok(paragraph_summary) => paragraph_summary,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };
    let response = KronosResponse {
        kronos_request: req.into_inner(),
        plans_vec: None,
        orders_vec: None,
        paragraphs_vec: Some(vec![paragraph_summary]),
        units_vec: None,
    };

    Ok(response)

}

fn check_delete_paragraph_request(
    req: &Json<KronosRequest>,
) -> Result<DeleteParagraphParams, KronosApiError> {
    //Check that there is a paragraph_reqeust struct
    let paragraph_request = match &req.paragraph_request {
        Some(paragraph_request) => paragraph_request,
        None => {
            return Err(KronosApiError::BadRequest(
                "No ParagraphRequest included in KronosRequest for delete_paragraph method."
                    .to_string(),
            ))
        }
    };
    //Check that there is a target paragraph id
    let paragraph_id = match &paragraph_request.paragraph_id {
        Some(paragraph_id) => paragraph_id,
        None => {
            return Err(KronosApiError::BadRequest(
                "No paragraph_id included in ParagraphRequest for delete_paragraph method."
                    .to_string(),
            ))
        }
    };

    Ok(DeleteParagraphParams {
        paragraph_id: paragraph_id,
    })
}

