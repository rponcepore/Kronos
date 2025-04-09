//!build_order_summary.rs

use sea_orm::*;

use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::*;
use crate::routes::api::helper_methods::build_order_summary::kronos_order_summary::KronosOrderSummary;
use crate::routes::api::helper_methods::build_order_summary::paragraph_summary::ParagraphSummary;
use crate::routes::api::helper_methods::build_paragraph_summary::*;
use crate::routes::api::parameters::network_structs::KronosApiError;

pub async fn build_order_summary(
    order: &kronos_order::Model,
    db: &DatabaseConnection,
) -> Result<KronosOrderSummary, KronosApiError> {
    // Repeated, recursive database calls...
    let mut kronos_order_summary = KronosOrderSummary {
        data: order.clone(),
        paragraphs: Some(Vec::<ParagraphSummary>::new()),
    };

    // Get all paragraphs that have paragraph.KronosOrder = order.id
    let result = match paragraph::Entity::find()
        .filter(paragraph::Column::KronosOrder.eq(order.id))
        .all(db)
        .await
    {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    // For each paragraph returned, convert it into a ParagraphSummary.
    for paragraph in result {
        let paragraph_summary = match build_paragraph_summary(&paragraph, db).await {
            Ok(paragraph_summary) => paragraph_summary,
            Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
        };
        kronos_order_summary.paragraphs.as_mut().expect("Unwrapped a NONE when SOME was explicitly declared above. I'm panicking with the routine.").push(paragraph_summary);
    }

    Ok(kronos_order_summary)
}
