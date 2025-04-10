//! create_order.rs

// This file defines the internal actions for creating a new order and populating it to the database.

use actix_web::web::Json;
use debug_print::debug_println as dprintln;
use sea_orm::*;

use crate::routes::api::helper_methods::build_order_import::{
    make_standard_order, make_standard_fragord, ImportParagraph,
};
use crate::routes::api::helper_methods::build_order_summary::build_order_summary;
use crate::routes::api::helper_methods::summarizers::*;
use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;

use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::kronos_order_summary::KronosOrderSummary;


use crate::models::entities::plan;

struct CreateOrderParams<'a> {
    plan_id: &'a i32,
    order_type: String,
}

pub async fn create_order(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    dprintln!("create_order called.");

    // Parse the request
    let checked_params = check_create_order_params(&req)?;

    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // If this is a WARNORD or a FRAGORD, we just need to increment the current count for that thing,
    // and then number it accordingly. If it is an OPORD, then we need to make sure that there is only one OPORD.
    // Ignore the return value, sinc'e we're going to return the plan anyway. 
    // (Is this a good idea? Feels a little redundant/unnecessary.)
    let _order_summary_wrapped = match checked_params.order_type.as_str() {
        "OPORD" => create_opord(&checked_params, &db).await,
        "FRAGORD" => create_fragord(&checked_params, &db).await,
        "WARNORD" => create_warnord(&checked_params, &db).await,
        _ => {
            return Err(KronosApiError::BadRequest(format!(
                "Illegal order_type specified: {}",
                checked_params.order_type
            )))
        }
    };

    // Go get the parent plan again. We will resend the entire plan to the frontend.
    let parent_plan = match Plan::find_by_id(checked_params.plan_id.to_owned())
        .one(&db)
        .await
        {
            Ok(parent_plan_option) => match parent_plan_option {
                Some(parent_plan) => parent_plan,
                None => return Err(KronosApiError::ExpectedDataNotPresent("While creating an order, the plan owning the order could not be found. Suggest panicking.".to_string())),
            },
            Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
        };

    let plan_summary = pack_plan_summary_deep(parent_plan, &db).await?;

    // Unwrap json<KronosRequest> into just a KronosRequest to avoid de-re-de-se-re-serialization issues.
    let plain_kronos_request = req.into_inner();

    // Encode them into a KronosResponse Object
    let kronos_response = KronosResponse {
        kronos_request: plain_kronos_request,
        plans_vec: Some(vec![plan_summary]),
        orders_vec: None,
        paragraphs_vec: None,
        units_vec: None,
    };
    // Send back to the client
    Ok(kronos_response)
}

fn check_create_order_params(
    req: &Json<KronosRequest>,
) -> Result<CreateOrderParams, KronosApiError> {
    // make sure that order_request is not null
    let order_req = match &req.order_request {
        Some(order_req) => order_req,
        None => {
            return Err(KronosApiError::BadRequest(
                "Request to create an order did not include an OrderRequest object.".to_string(),
            ))
        }
    };

    let plan_id = match &order_req.parent_plan_id {
        Some(plan_id) => plan_id,
        None => return Err(KronosApiError::BadRequest(
            "Request to create an order did not include an plan_id field in the OrderRequest object.".to_string(),
        )),
    };

    let order_type = match &order_req.order_type {
        Some(order_type) => match order_type.as_str() {
            "WARNORD" => "WARNORD".to_string(),
            "OPORD" => "OPORD".to_string(),
            "FRAGORD" => "FRAGORD".to_string(),
            _ => return Err(
                KronosApiError::BadRequest(
                    format!(
                        "Unknown order type supplied. Options are WARNORD, FRAGORD, and OPORD. \
                        You supplied {}, which is not allowed.", 
                        order_type.as_str()
                    )
                )
            ),
        }
        None => return Err(
            KronosApiError::BadRequest(
                "Request to create an order did not include an order_type field in the OrderRequest object."
                .to_string()
            )
        )
    };

    let result: CreateOrderParams<'_> = CreateOrderParams {
        plan_id: plan_id,
        order_type: order_type,
    };

    Ok(result)
}

async fn create_opord(
    params: &CreateOrderParams<'_>,
    db: &DatabaseConnection,
) -> Result<KronosOrderSummary, KronosApiError> {
    // There can only be one OPORD. Check this now.
    let plan = get_plan_model(params.plan_id, db).await?;

    // check to see if there are any opords (there should only be one!)
    let orders_vec = match KronosOrder::find()
        .filter(kronos_order::Column::ParentPlan.eq(params.plan_id.to_owned()))
        .filter(kronos_order::Column::OrderType.eq("OPORD"))
        .all(db)
        .await
    {
        Ok(orders_vec) => orders_vec,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    match orders_vec.len() {
        0 => {} // base case, we need to create an order
        1 => {
            return Err(KronosApiError::BadRequest(
                "Client is requesting to create a new OPORD for a plan that already has an OPORD."
                    .to_string(),
            ))
        }
        _ => {
            return Err(KronosApiError::Unknown(format!(
                "This plan indicates that it has multiple ({}) OPORDs associated with it.",
                orders_vec.len()
            )))
        }
    };

    let new_order = kronos_order::ActiveModel {
        parent_plan: ActiveValue::Set(params.plan_id.to_owned()),
        order_type: ActiveValue::Set(params.order_type.to_owned()),
        serial_number: ActiveValue::NotSet,
        is_published: ActiveValue::Set(false),
        derived_from: ActiveValue::NotSet,
        ..Default::default() //do not set auto incrementing uuid pk
    };

    // Send it to the database
    let result = match new_order.insert(db).await {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    // This is an OPORD, so build it with the standard opord contents.
    let opord: kronos_order::Model = attach_standard_opord_paragraphs(result.id, db).await?;

    // Now pack this order and return it as an order summary
    let order_summary = build_order_summary(&opord, db).await?;

    Ok(order_summary)
}

//For creating either FRAGORDs or WARNORDS
async fn create_warnord(
    params: &CreateOrderParams<'_>,
    db: &DatabaseConnection,
) -> Result<KronosOrderSummary, KronosApiError> {
    //get the plan
    let plan = get_plan_model(params.plan_id, db).await?;

    // WARNORDS are mercifully easy. Five paragraphs.
    // First, deduce the number of the warnord.
    let warnords_vec = match KronosOrder::find()
        .filter(kronos_order::Column::ParentPlan.eq(params.plan_id.to_owned()))
        .filter(kronos_order::Column::OrderType.eq("WARNORD"))
        .order_by_desc(kronos_order::Column::SerialNumber)
        .all(db)
        .await
    {
        Ok(warnords_vec) => warnords_vec,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    let mut highest_known_serial = 0;
    let vec_length = (&warnords_vec.len()).clone();
    if vec_length > 0 {
        let latest_warnord = &warnords_vec[vec_length-1];
        highest_known_serial = latest_warnord.serial_number;
    }

    let new_serial_number = highest_known_serial+1;

    //Create the order
    let new_order = kronos_order::ActiveModel {
        parent_plan: ActiveValue::Set(params.plan_id.to_owned()),
        order_type: ActiveValue::Set("FRAGORD".to_owned()),
        serial_number: ActiveValue::Set(new_serial_number),
        is_published: ActiveValue::Set(false),
        derived_from: ActiveValue::NotSet,
        ..Default::default() //do not set auto incrementing uuid pk
    };

    // Send it to the database
    let result = match new_order.insert(db).await {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    // This is a warnord, so only add the first five paragraphs.
    let fragord_import = make_standard_fragord()?;

    // Start building out paragraphs
    for paragraph in &fragord_import.paragraphs {
        // Create a paragraph in the database, return it's id
        let paragraph_model = add_major_paragraph_to_db(&result.id, &paragraph, db).await?;
    }

    // Now pack this order and return it as an order summary
    let order_summary = build_order_summary(&result, db).await?;

    Ok(order_summary)


}


//For creating either FRAGORDs or WARNORDS
async fn create_fragord(
    params: &CreateOrderParams<'_>,
    db: &DatabaseConnection,
) -> Result<KronosOrderSummary, KronosApiError> {
    
    Err(KronosApiError::NotImplemented("New FRAGORDS are not implemented at this time.".to_string()))
}

async fn attach_standard_opord_paragraphs(
    order_id: i32,
    db: &DatabaseConnection,
) -> Result<kronos_order::Model, KronosApiError> {
    // Read in from yaml file.
    let order_import = make_standard_order()?;

    // Start building out paragraphs
    for paragraph in &order_import.paragraphs {
        // Create a paragraph in the database, return it's id
        let paragraph_model = add_major_paragraph_to_db(&order_id, &paragraph, db).await?;
        let para_id = paragraph_model.id;
        // Now check if the Import paragraph has subparagraphs. If it did, repeat ad nauseum.
        let indent_level = 1;
        for subparagraph in &paragraph.subparagraphs {
            let _new_paragraph =
                add_subparagraph_to_db(&para_id, &order_id, &indent_level, &subparagraph, db)
                    .await?;
        }
    }

    let order_model =
        match KronosOrder::find_by_id(order_id).one(db).await {
            Ok(order_model) => match order_model {
                Some(order_model) => order_model,
                None => return Err(KronosApiError::ExpectedDataNotPresent(
                    "Although a new order was just inserted, the database cannot find it. Panic?"
                        .to_string(),
                )),
            },
            Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
        };
    
    Ok(order_model)
}

async fn add_major_paragraph_to_db(
    order_id: &i32,
    paragraph: &ImportParagraph,
    db: &DatabaseConnection,
) -> Result<paragraph::Model, KronosApiError> {
    // Build the SeaORM query
    let new_paragraph = paragraph::ActiveModel {
        id: ActiveValue::default(),
        title: ActiveValue::Set(paragraph.title.to_owned()),
        text: ActiveValue::Set(paragraph.text.to_owned()),
        kronos_order: ActiveValue::Set(order_id.to_owned()),
        parent_paragraph: ActiveValue::Set(None),
        ordinal_sequence: ActiveValue::Set(paragraph.ordinal_sequence.to_owned()),
        indent_level: ActiveValue::Set(0),
    };

    // Execute the insertion.
    let result_paragraph = match new_paragraph.insert(db).await {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };
    Ok(result_paragraph)
}

async fn add_subparagraph_to_db(
    parent_paragraph_id: &i32,
    order_id: &i32,
    indent_level: &i32,
    paragraph: &ImportParagraph,
    db: &DatabaseConnection,
) -> Result<paragraph::Model, KronosApiError> {
    let new_paragraph = paragraph::ActiveModel {
        id: ActiveValue::default(),
        title: ActiveValue::Set(paragraph.title.to_owned()),
        text: ActiveValue::Set(paragraph.text.to_owned()),
        kronos_order: ActiveValue::Set(order_id.to_owned()),
        parent_paragraph: ActiveValue::Set(Some(parent_paragraph_id.to_owned())),
        ordinal_sequence: ActiveValue::Set(paragraph.ordinal_sequence.to_owned()),
        indent_level: ActiveValue::Set(indent_level.to_owned()),
    };

    // Execute the insertion.
    let result_paragraph = match new_paragraph.insert(db).await {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };
    let child_indent_level = indent_level + 1;
    for subparagraph in &paragraph.subparagraphs {
        let _ignored_value = Box::pin(async move {
            add_subparagraph_to_db(
                &result_paragraph.id,
                order_id,
                &child_indent_level,
                &subparagraph,
                db,
            )
            .await
        }).await?;
        
    }

    Ok(result_paragraph)
}

async fn get_plan_model(plan_id: &i32, db: &DatabaseConnection) -> Result<plan::Model, KronosApiError> {
    let plan: plan::Model = match Plan::find()
        .filter(plan::Column::Id.eq(plan_id.clone()))
        .one(db)
        .await
    {
        Ok(plan) => match plan {
            Some(plan) => plan,
            None => {
                return Err(KronosApiError::ExpectedDataNotPresent(format!(
                    "Could not find a plan record that matched plan_id {}",
                    plan_id
                )))
            }
        },
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };
    Ok(plan)
}