//! paragraph_helper_methods.rs

// This file defines convenient reused code in the paragraph API.
// Future refactors should look for opportunities to include it

use sea_orm::*;

use crate::routes::api::parameters::network_structs::*;

use crate::models::entities::{prelude::*, *};

pub async fn get_target_record(
    paragraph_id: &i32,
    db: &DatabaseConnection,
) -> Result<paragraph::Model, KronosApiError> {
    // Get target paragraph record; what a workout.
    let target_paragraph_record = match Paragraph::find_by_id(paragraph_id.clone()).one(db).await {
        Ok(target_paragraph_model) => match target_paragraph_model {
            Some(target_paragraph_record) => target_paragraph_record,
            None => {
                return Err(KronosApiError::ExpectedDataNotPresent(format!(
                    "Database has no entry for paragraph with id {}",
                    &paragraph_id
                )))
            }
        },
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    Ok(target_paragraph_record)
}

// This method gets a parent paragraph of a target paragraph using a pre-connected database connection.
pub async fn get_parent_paragraph(
    target_paragraph: &paragraph::Model,
    db: &DatabaseConnection,
) -> Result<paragraph::Model, KronosApiError> {
    let parent_paragraph_id = match target_paragraph.parent_paragraph {
        Some(parent_paragraph_id) => parent_paragraph_id,
        None => {
            return Err(KronosApiError::ExpectedDataNotPresent(format!(
                "In attempting to locate the parent paragraph of the target paragraph, \
                 Kronos could not find the parent's id in the target paragraph's data. \
                 This means that either (1) you have made a call to insert a sibling \
                 paragraph on a major paragraph, or (2) to delete a major paragraph, \
                 neither of which is allowed. Aternatively, an invariant \
                 of paragraph data has been violated, in which case, panic."
            )))
        }
    };
    let parent_option: Option<paragraph::Model> =
        match Paragraph::find_by_id(parent_paragraph_id.to_owned())
            .one(db)
            .await
        {
            Ok(parent_option) => parent_option,
            Err(msg) => return Err(KronosApiError::DbErr(msg)),
        };

    let parent = match parent_option {
        Some(parent) => parent,
        None => {
            return Err(KronosApiError::ExpectedDataNotPresent(format!(
                "In attempting to locate the parent paragraph of the target paragraph, \
             Kronos could not find the parent's record, despite the child paragraph \
             recording a valid integer primary key. This is not due to a database \
             connecton failure, but rather to a foreign key pointing to nothing. Panic."
            )))
        }
    };

    Ok(parent)
}

// This function updates the ordinal sequence of a paragraph to be one greater than it currently is.
// Useful in cascading updates of ordinal sequence
pub async fn adjust_ordinal_sequence(
    direction: i32,
    paragraph: paragraph::Model,
    db: &DatabaseConnection,
) -> Result<paragraph::Model, KronosApiError> {
    // Build the SeaORM query
    let updated_paragraph = paragraph::ActiveModel {
        id: ActiveValue::Set(paragraph.id.to_owned()), //no change
        title: ActiveValue::NotSet,
        text: ActiveValue::NotSet,
        kronos_order: ActiveValue::NotSet,
        parent_paragraph: ActiveValue::NotSet,
        ordinal_sequence: ActiveValue::Set(paragraph.ordinal_sequence + direction),
        indent_level: ActiveValue::NotSet,
    };

    // Execution
    let result = match updated_paragraph.update(db).await {
        Ok(result) => result,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    Ok(result)
}
