//! get_plans.rs

use actix_web::web::Json;
use sea_orm::*;
use debug_print::debug_println as dprintln;

use crate::models::entity_summaries::kronos_order_summary;
use crate::models::entity_summaries::kronos_order_summary::KronosOrderSummary;
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;
use crate::models::entity_summaries::plan_summary::PlanSummary;
use crate::routes::api::api_handler::KronosApiError;
use crate::routes::api::api_handler::KronosRequest;
use crate::routes::api::api_handler::KronosResponse;
use crate::routes::api::api_handler::access_kronos_database;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};

pub async fn get_order(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError>  {
    dprintln!("get_orders method called. ");

    // Connect to the database
    let db = match access_kronos_database().await{
        Ok(db) => db,
        Err(error) => return Err(KronosApiError::DbErr(error)),
    };

    let unit_str = match &req.unit {
        Some(unit) => unit.as_str(),
        None => return Err(KronosApiError::Unknown("Deserialization error: unit string failure.".to_string())),
    };

    let order_id: i32 = match &req.order_id {
        Some(order_id) => *order_id,
        None => return Err(KronosApiError::BadRequest("No order_id field provided with get_order request.".to_string())),
    };

    // We are being asked for the data within an order.
    // Get an order, serialize into an OrderSummary, and send back to the client.
    let order: Option<kronos_order::Model> = match KronosOrder::find_by_id(order_id).one(&db).await
        {
            Ok(order) => order,
            Err(msg) => return Err(KronosApiError::DbErr(msg)),
        };
    
    let kronos_order = match order {
        Some(order) => order,  // Use `order` directly, not `kronos_order`
        None => return Err(KronosApiError::DbErr(DbErr::RecordNotFound(
            format!("The request for order_id {} returned 0 results. It may have been deleted.", &order_id)
        ))),
    };
    
    // We now have the model itself of kronos_order. Now we need to construct an order summary from this order.
    let order_summary = match build_order_summary(&kronos_order, &db).await{
        Ok(order_summary) => order_summary,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // Unwrap json<KronosRequest> into just a KronosRequest to avoid de-re-de-se-re-serialization issues. 
    let plain_kronos_request = req.into_inner();
    
    // Encode them into a KronosResponse Object
    let kronos_response = KronosResponse {
        kronos_request: plain_kronos_request,
        plans_vec: None,
        orders_vec: Some(vec![order_summary]),
        paragraphs_vec: None,
        units_vec: None,
    };
    // Send back to the client
    Ok(kronos_response)
}

async fn build_order_summary(order: &kronos_order::Model, db: &DatabaseConnection) -> Result<KronosOrderSummary, DbErr> {

    // Repeated, recursive database calls...
    let mut kronos_order_summary = KronosOrderSummary{
        data: order.clone(),
        paragraphs: Some(Vec::<ParagraphSummary>::new()),
    };

    // Get all paragraphs that have paragraph.KronosOrder = order.id
    let result = paragraph::Entity::find()
        .filter(paragraph::Column::KronosOrder.eq(order.id))
        .all(db)
        .await?;

    // For each paragraph returned, convert it into a ParagraphSummary. 
    for paragraph in result{
        let paragraph_summary = assemble_paragraph(&paragraph, db).await?;
        kronos_order_summary.paragraphs.as_mut().expect("Unwrapped a NONE when SOME was explicitly declared above. I'm panicking with the routine.").push(paragraph_summary);
    }
    
    Ok(kronos_order_summary)

}

async fn assemble_paragraph(paragraph: &paragraph::Model, db: &DatabaseConnection) -> Result<ParagraphSummary, DbErr> {
    let mut paragraph_summary = ParagraphSummary{
        data: paragraph.clone(),
        subparagraphs: None,
    };

    // Get all paragraphs that have paragraph.ParentParagraph = paragraph.id
    let result = paragraph::Entity::find()
        .filter(paragraph::Column::ParentParagraph.eq(paragraph.id))
        .all(db)
        .await?;

    match result.len() {
        0 => {},
        _ => {
            paragraph_summary.subparagraphs = Some(Vec::<ParagraphSummary>::new());
            for subparagraph in result{
                let subparagraph_summary = Box::pin(async move {
                    assemble_paragraph(&subparagraph, db)
                    .await
                }).await?;
                paragraph_summary.subparagraphs.as_mut().expect("Unwrapped NONE when SOME was explicitly declared in get_order.rs").push(subparagraph_summary);
            };
        },
    };
    
    Ok(paragraph_summary)
}

