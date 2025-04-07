//! get_plans.rs

use actix_web::web::Json;
use sea_orm::*;
use debug_print::debug_println as dprintln;

use crate::models::entity_summaries::kronos_order_summary::KronosOrderSummary;
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;
use crate::models::entity_summaries::plan_summary::PlanSummary;
use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;

//helper methods
use crate::routes::api::helper_methods::assemble_paragraph_summary::*;
// Pull in our entities,
use crate::models::entities::{prelude::*, *};

pub async fn get_order(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError>  {
    dprintln!("get_orders method called. ");

    // Connect to the database
    let db = match access_kronos_database().await{
        Ok(db) => db,
        Err(error) => return Err(KronosApiError::DbErr(error)),
    };

    let unit_str = match &req.uic {
        Some(uic) => uic.as_str(),
        None => return Err(KronosApiError::Unknown("Deserialization error: uic string failure.".to_string())),
    };

    let order_id = match &req.order_request{
        Some(order_request) => match &order_request.order_id {
            Some(order_id) => order_id,
            None => return Err(KronosApiError::BadRequest("No order_id field provided with get_order request.".to_string()))
        },
        None => return Err(KronosApiError::BadRequest("User did not provide an order request to initialize plan.".to_string())),
    };

    // We are being asked for the data within an order.
    // Get an order, serialize into an OrderSummary, and send back to the client.
    let order: Option<kronos_order::Model> = match KronosOrder::find_by_id(order_id.clone()).one(&db).await
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
        let paragraph_summary = assemble_paragraph_summary(&paragraph, db).await?;
        kronos_order_summary.paragraphs.as_mut().expect("Unwrapped a NONE when SOME was explicitly declared above. I'm panicking with the routine.").push(paragraph_summary);
    }
    
    Ok(kronos_order_summary)

}


