//! get_plans.rs

use actix_web::web::Json;
use debug_print::debug_println as dprintln;
use sea_orm::*;

use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;

//helper methods
use crate::routes::api::helper_methods::build_order_summary::*;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};

pub async fn get_order(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    dprintln!("get_orders method called. ");

    // Connect to the database
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(error) => return Err(KronosApiError::DbErr(error)),
    };

    let _unit_str = match &req.uic {
        Some(uic) => uic.as_str(),
        None => {
            return Err(KronosApiError::Unknown(
                "Deserialization error: uic string failure. Request appears not to have a UIC included.".to_string(),
            ))
        }
    };

    let order_id = match &req.order_request {
        Some(order_request) => match &order_request.order_id {
            Some(order_id) => order_id,
            None => {
                return Err(KronosApiError::BadRequest(
                    "No order_id field provided with get_order request.".to_string(),
                ))
            }
        },
        None => {
            return Err(KronosApiError::BadRequest(
                "User did not provide an order request to initialize plan.".to_string(),
            ))
        }
    };

    // We are being asked for the data within an order.
    // Get an order, serialize into an OrderSummary, and send back to the client.
    let order: Option<kronos_order::Model> =
        match KronosOrder::find_by_id(order_id.clone()).one(&db).await {
            Ok(order) => order,
            Err(msg) => return Err(KronosApiError::DbErr(msg)),
        };

    let kronos_order = match order {
        Some(order) => order, // Use `order` directly, not `kronos_order`
        None => {
            return Err(KronosApiError::DbErr(DbErr::RecordNotFound(format!(
                "The request for order_id {} returned 0 results. It may have been deleted.",
                &order_id
            ))))
        }
    };

    // We now have the model itself of kronos_order. Now we need to construct an order summary from this order.
    let order_summary = build_order_summary(&kronos_order, &db).await?;

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
