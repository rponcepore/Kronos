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
use debug_print::debug_println as dprintln;
use sea_orm::*;

use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;

use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;

use crate::routes::api::api_methods::paragraph_api::paragraph_helper_methods::*;
use crate::routes::api::helper_methods::build_paragraph_summary::*;

struct InsertParagraphParams<'a> {
    paragraph_id: &'a i32,
    insert_method: &'a String,
    new_text: &'a String,
    new_title: &'a String,
}

// The request has already been vetted for general compliance, this fn will handle only the special cases.
pub async fn insert_paragraph(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    dprintln!("Insert_paragraph called.");

    let checked_params = check_insert_paragraph_request(&req)?;
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // Identify the target paragraph
    let target_paragraph_record = match get_target_record(checked_params.paragraph_id, &db).await {
        Ok(target_paragraph_record) => target_paragraph_record,
        Err(msg) => return Err(msg),
    };

    let siblingship = checked_params.insert_method.as_str();

    // Check a quick invariant: No calls to insert siblings on major paragraphs.
    if (target_paragraph_record.indent_level == 0) && (siblingship != "SUBPARAGRAPH") {
        return Err(KronosApiError::BadRequest(format!(
            "You cannot insert a sibling paragraph on a major paragraph; \
                    you may insert subpararaphs on major paragraphs. (i.e., it's a five \
                    paragraph OPORD, and KRONOS enforces this."
        )));
    }

    // Match the case to the function.
    // These variants are defined on teh frontend as an enum: string (typescript) as
    // insert_method: "ABOVE" | "BELOW" | "SUBPARAGRAPH" | null,
    let paragraph_summary_res: Result<ParagraphSummary, KronosApiError> = match checked_params
        .insert_method
        .as_str()
    {
        "ABOVE" => insert_sibling(true, checked_params, &target_paragraph_record, &db).await,
        "BELOW" => insert_sibling(false, checked_params, &target_paragraph_record, &db).await,
        "SUBPARAGRAPH" => insert_subparagraph(checked_params, &target_paragraph_record, &db).await,
        _ => Err(KronosApiError::BadRequest(format!(
            "Unknown insert_method included on insert_paragraph request: {}",
            checked_params.insert_method
        ))),
    };

    let paragraph_summary = match paragraph_summary_res {
        Ok(paragraph_summary) => paragraph_summary,
        Err(msg) => return Err(msg),
    };

    let kronos_response = KronosResponse::new(req).with_paragraph(paragraph_summary);

    Ok(kronos_response)
}

fn check_insert_paragraph_request(
    req: &Json<KronosRequest>,
) -> Result<InsertParagraphParams, KronosApiError> {
    //Check that there is a paragraph_reqeust struct
    let paragraph_request = match &req.paragraph_request {
        Some(paragraph_request) => paragraph_request,
        None => {
            return Err(KronosApiError::BadRequest(
                "No ParagraphRequest included in KronosRequest for insert_paragraph method."
                    .to_string(),
            ))
        }
    };
    //Check that there is a target paragraph id
    let paragraph_id = match &paragraph_request.paragraph_id {
        Some(paragraph_id) => paragraph_id,
        None => {
            return Err(KronosApiError::BadRequest(
                "No paragraph_id included in ParagraphRequest for insert_pargraph method."
                    .to_string(),
            ))
        }
    };
    //Check that there is a valid insert_method
    let insert_method = match &paragraph_request.insert_method {
        Some(insert_method) => insert_method,
        None => {
            return Err(KronosApiError::BadRequest(
                "No insert_method included in ParagraphRequest for insert_paragraph method."
                    .to_string(),
            ))
        }
    };
    // Need new text
    let new_text = match &paragraph_request.new_text {
        Some(new_text) => new_text,
        None => {
            return Err(KronosApiError::BadRequest(
                "No new_text included in ParagraphRequest for insert_paragraph method.".to_string(),
            ))
        }
    };

    // Need new title
    let new_title = match &paragraph_request.new_title {
        Some(new_title) => new_title,
        None => {
            return Err(KronosApiError::BadRequest(
                "No new_title included in ParagraphRequest for insert_paragraph method."
                    .to_string(),
            ))
        }
    };

    Ok(InsertParagraphParams {
        paragraph_id: paragraph_id,
        insert_method: insert_method,
        new_text: new_text,
        new_title: new_title,
    })
}

async fn find_greatest_paragraph_number(
    target_paragraph_record: &paragraph::Model,
    db: &DatabaseConnection,
) -> Result<i32, KronosApiError> {
    let subparagraph_vec: Vec<paragraph::Model> = match Paragraph::find()
        .filter(paragraph::Column::ParentParagraph.eq(target_paragraph_record.id))
        .order_by_desc(paragraph::Column::OrdinalSequence)
        .all(db)
        .await
    {
        Ok(subparagraph_vec) => subparagraph_vec,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    match subparagraph_vec.len() {
        0 => Ok(0),
        _ => Ok(subparagraph_vec[0].ordinal_sequence),
    }
}

async fn insert_sibling(
    insert_above_case: bool,
    checked_params: InsertParagraphParams<'_>,
    target_paragraph_record: &paragraph::Model,
    db: &DatabaseConnection,
) -> Result<ParagraphSummary, KronosApiError> {
    // There are two cases. Either insert above the target, or below the target.
    // In either case, the best way to resolve it is just to figure out the
    // new paragraph's ordinal sequence, and then adjusting every paragraph below it's ordinal sequence,
    // then inserting the new paragraph.
    // at the end of the function it's probably a good idea to assert that everything is still in the correct order
    // This might (TODO) later come to haunt us if there's lots of simultaneous editors.

    // Find what the new paragraph's sequence needs to be
    let new_ordinal_sequence = match insert_above_case {
        // In the true case, we need to insert the paragraph above. So give it the same ordinal sequence as the
        // target, and adjust everything else down.
        true => target_paragraph_record.ordinal_sequence,
        // In the false case, we need to insert the paragraph below.
        // Give it an ordinal sequence once greater than the target, and push everything else down.
        false => target_paragraph_record.ordinal_sequence + 1,
    };

    // Get the parent paragraph
    let parent = get_parent_paragraph(target_paragraph_record, db).await?;

    // Find all paragraphs potentially affected
    let para_vec: Vec<paragraph::Model> = match Paragraph::find()
        .filter(paragraph::Column::ParentParagraph.eq(parent.id))
        .all(db)
        .await
    {
        Ok(para_vec) => para_vec,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    //Cascade update all the paragraphs below it to add one to their numbers
    for paragraph in para_vec {
        //check if the ordinal sequence is equal to or greater than the ordinal sequence we need to insert
        // if yes, +=1 , else ignore
        if paragraph.ordinal_sequence >= new_ordinal_sequence {
            // Ensure we send this to the database!
            adjust_ordinal_sequence(1, paragraph, db).await?;
        }
    }

    // Finally, create the incoming paragraph and add it.
    let new_paragraph = paragraph::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(checked_params.new_title.to_owned()),
        text: ActiveValue::Set(checked_params.new_text.to_owned()),
        kronos_order: ActiveValue::Set(target_paragraph_record.kronos_order),
        parent_paragraph: ActiveValue::Set(target_paragraph_record.parent_paragraph),
        ordinal_sequence: ActiveValue::Set(new_ordinal_sequence),
        indent_level: ActiveValue::Set(target_paragraph_record.indent_level),
    };

    let _result = match new_paragraph.insert(db).await {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    // Finally, get the parent paragraph, wrap it, and return it.
    let updated_parent = get_parent_paragraph(target_paragraph_record, db).await?;
    let paragraph_summary = match build_paragraph_summary(&updated_parent, db).await {
        Ok(paragraph_summary) => paragraph_summary,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };
    Ok(paragraph_summary)
}

async fn insert_subparagraph(
    checked_params: InsertParagraphParams<'_>,
    target_paragraph_record: &paragraph::Model,
    db: &DatabaseConnection,
) -> Result<ParagraphSummary, KronosApiError> {
    dprintln!("Executing logic for inserting a subparagraph.");
    // Find the last paragraph, if it exists, and get it's number
    let greatest_subparagraph_number =
        find_greatest_paragraph_number(&target_paragraph_record, db).await?;

    let new_subparagraph_number = greatest_subparagraph_number + 1;

    // Build the SeaORM query
    let new_paragraph = paragraph::ActiveModel {
        id: ActiveValue::default(),
        title: ActiveValue::Set(checked_params.new_title.to_owned()),
        text: ActiveValue::Set(checked_params.new_text.to_owned()),
        kronos_order: ActiveValue::Set(target_paragraph_record.kronos_order),
        parent_paragraph: ActiveValue::Set(Some(target_paragraph_record.id.to_owned())),
        ordinal_sequence: ActiveValue::Set(new_subparagraph_number),
        indent_level: ActiveValue::Set(&target_paragraph_record.indent_level + 1),
    };

    // Execute the insertion.

    let result = match new_paragraph.insert(db).await {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    // Return the parent paragraph
    let parent = get_parent_paragraph(&result, db).await?;

    // Build paragraph summary
    let paragraph_summary: ParagraphSummary = match build_paragraph_summary(&parent, db).await {
        Ok(paragraph_summary) => paragraph_summary,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    Ok(paragraph_summary)
}
