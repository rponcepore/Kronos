//! get_plans.rs

use actix_web::web::Json;
use sea_orm::*;
use debug_print::debug_println as dprintln;

use crate::models::entity_summaries::kronos_order_summary::KronosOrderSummary;
use crate::models::entity_summaries::plan_summary::PlanSummary;
use crate::routes::api::api_handler::KronosApiError;
use crate::routes::api::api_handler::KronosRequest;
use crate::routes::api::api_handler::KronosResponse;
use crate::utilities::database_tools::access_kronos_database;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};

pub async fn get_plans(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError>  {
    dprintln!("get_plans method called. ");

    // TODO: Delete this bad, very bad, hack, that is used only for development:
    if req.unit.as_deref().unwrap_or("") == "tstUIC" { // This is a same unwrap because unit was already checked for None
        let plan_vec = vec![
            plan::Model {
                id: 1,
                unit: "WJH8C0".to_string(),
                parent_plan: None,
                fiscal_year: 2025,
                serial_number: 1,
                classification: "Top Secret".to_string(),
                name: "Operation Blackbeard".to_string(),
            },
            plan::Model {
                id: 2,
                unit: "WJH8C0".to_string(),
                parent_plan: None,
                fiscal_year: 2025,
                serial_number: 2,
                classification: "CUI".to_string(),
                name: "Revenge Strategy".to_string(),
            },
            plan::Model {
                id: 3,
                unit: "WJH8C0".to_string(),
                parent_plan: None,
                fiscal_year: 2025,
                serial_number: 3,
                classification: "Secret".to_string(),
                name: "Jack Sparrow's Gambit".to_string(),
            },
        ];
        let mut sum_vec: Vec<PlanSummary> = Vec::new();
        for plan in plan_vec {
            let plan_summary: PlanSummary = PlanSummary { data: plan, orders: None };
            sum_vec.push(plan_summary);
        }
        let kronos_response = KronosResponse {
            kronos_request: req.into_inner(),
            plans_vec: Some(sum_vec),
            orders_vec: None,
            paragraphs_vec: None,
            units_vec: None,
        };
        return Ok(kronos_response);
    } //END of teh very bad hack that should be deleted. (I need to install mocking, oof.)

    // NORMAL execution:
    // Connect to the database
    let db = match access_kronos_database().await{
        Ok(db) => db,
        Err(error) => return Err(KronosApiError::DbErr(error)),
    };

    let unit_str = match &req.unit {
        Some(unit) => unit.as_str(),
        None => return Err(KronosApiError::Unknown("Deserialization error: unit string failure.".to_string())),
    };

    // Get all the plans for that unit
    let plan_vec: Vec<plan::Model> = match Plan::find()
        .filter(plan::Column::Unit.contains(unit_str))
        .order_by_asc(plan::Column::FiscalYear)
        .order_by_asc(plan::Column::SerialNumber)
        .all(&db)
        .await {
            Ok(plan_vec) => plan_vec,
            Err(msg) => return Err(KronosApiError::DbErr(msg)),
        };

    
    let mut plan_summary_vec: Vec<PlanSummary> = Vec::new();
    // For each plan returned, get it's associated orders
    for plan in plan_vec {

        let packed_plan_summary = match pack_plan_summary(plan, &db).await {
            Ok(packed_plan_summary) => packed_plan_summary,
            Err(err) => return Err(err),
        };
        plan_summary_vec.push(packed_plan_summary);
    }

    // Unwrap json<KronosRequest> into just a KronosRequest to avoid de-re-de-se-re-serialization issues. 
    let plain_kronos_request = req.into_inner();
    
    // Encode them into a KronosResponse Object
    let kronos_response = KronosResponse {
        kronos_request: plain_kronos_request,
        plans_vec: Some(plan_summary_vec),
        orders_vec: None,
        paragraphs_vec: None,
        units_vec: None,
    };
    // Send back to the client
    Ok(kronos_response)
}

async fn pack_plan_summary(plan: plan::Model, db: &DatabaseConnection) -> Result<PlanSummary, KronosApiError> {

    //let planid:i32 = plan.clone().id;

    // Check the database for orders
    let order_vec_single_plan: Vec<kronos_order::Model> = match KronosOrder::find()
            .filter(kronos_order::Column::ParentPlan.eq(plan.id))
            .order_by_asc(kronos_order::Column::SerialNumber)
            .all(db)
            .await {
                Ok(order_vec_single_plan) => order_vec_single_plan,
                Err(msg) => return Err(KronosApiError::DbErr(msg)),
            };

    // Initialize the summary
    let mut plan_summary: PlanSummary = PlanSummary { data: plan, orders: None };
    
    // If there were orders, convert them to summaries
    match order_vec_single_plan.len() {
        0 => {},
        _ => {
            let mut packed_orders= Vec::<KronosOrderSummary>::new();
            for db_order in order_vec_single_plan {
                let packed_order = match pack_orders_summary(db_order, &db).await {
                    Ok(packed_order) => packed_order,
                    Err(err) => return Err(err),
                };
                packed_orders.push(packed_order);
            }
            plan_summary.orders = Some(packed_orders);
        },
    };
    
    // return the plan summary
    Ok(plan_summary)
}

async fn pack_orders_summary(order: kronos_order::Model, _db: &DatabaseConnection) -> Result<KronosOrderSummary, KronosApiError> {
    

    let paragraph_vec_single_plan: Vec<paragraph::Model> = Vec::<paragraph::Model>::new();
    // Need to make a call to the database to retrieve this!
    // TODO: go to the database, possibly with a recursve method, oh boy, no way that this will spiral out of control.

    let orders_summary: KronosOrderSummary = KronosOrderSummary { data:order, paragraphs: None };

    if paragraph_vec_single_plan.len() > 0 {
        // TODO: pack paragraph method
        // Spiral down, start grabbing the lower level data here!
    }

    Ok(orders_summary)
}