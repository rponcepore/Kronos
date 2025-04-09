//! paragraph_summary_builder.rs

// This file contains utility methods for building out a paragraph summary

use crate::models::entities::*;
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;
use sea_orm::*;

//Recursive method that builds a paragraphsumamry, including all child paragraphs, for a paragraph.
pub async fn build_paragraph_summary(
    paragraph: &paragraph::Model,
    db: &DatabaseConnection,
) -> Result<ParagraphSummary, DbErr> {
    let mut paragraph_summary = ParagraphSummary {
        data: paragraph.clone(),
        subparagraphs: None,
    };

    // Get all paragraphs that have paragraph.ParentParagraph = paragraph.id
    let result = paragraph::Entity::find()
        .filter(paragraph::Column::ParentParagraph.eq(paragraph.id))
        .all(db)
        .await?;

    match result.len() {
        0 => {}
        _ => {
            paragraph_summary.subparagraphs = Some(Vec::<ParagraphSummary>::new());
            for subparagraph in result {
                let subparagraph_summary =
                    Box::pin(async move { build_paragraph_summary(&subparagraph, db).await })
                        .await?;
                paragraph_summary
                    .subparagraphs
                    .as_mut()
                    .expect("Unwrapped NONE when SOME was explicitly declared in get_order.rs")
                    .push(subparagraph_summary);
            }
        }
    };

    Ok(paragraph_summary)
}
