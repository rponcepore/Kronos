//! get_plans.rs

// Basic imports
use actix_web::web::Json;
use sea_orm::*;
use debug_print::debug_println as dprintln;

// Network utilities
use crate::routes::api::parameters::network_structs::*;
use crate::utilities::database_tools::access_kronos_database;

// Pull in our entities,
use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::kronos_order_summary::KronosOrderSummary;
use crate::models::entity_summaries::plan_summary::PlanSummary;

pub async fn get_plans(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError>  {
    dprintln!("get_plans method called. ");

    // Connect to the database
    let db = match access_kronos_database().await{
        Ok(db) => db,
        Err(error) => return Err(KronosApiError::DbErr(error)),
    };

    let unit_str = match &req.uic {
        Some(uic) => uic.as_str(),
        None => return Err(KronosApiError::Unknown("Deserialization error: uic string failure.".to_string())),
    };

    // Get all the plans for that unit
    let plan_vec: Vec<plan::Model> = match Plan::find()
        .filter(plan::Column::Uic.contains(unit_str))
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
    let mut plan_summary: PlanSummary = PlanSummary { data: plan, orders: None , most_recent_mission: None};
    
    // If there were orders, convert them to summaries
    match order_vec_single_plan.len() {
        0 => {},
        _ => {
            let mut packed_orders= Vec::<KronosOrderSummary>::new();
            for db_order_ref in &order_vec_single_plan {
                let db_order = db_order_ref.clone();
                let packed_order = match pack_orders_summary(db_order, &db).await {
                    Ok(packed_order) => packed_order,
                    Err(err) => return Err(err),
                };
                packed_orders.push(packed_order);
            }
            plan_summary.orders = Some(packed_orders);
        },
    };

    // For the sake of the frontend, we need to send the msot recent mission statement. 
    let most_recent_mission_statement: Option<String> = match get_most_recent_mission(order_vec_single_plan, db).await {
        Ok(most_recent_mission_statement) => most_recent_mission_statement,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };
    
    plan_summary.most_recent_mission = most_recent_mission_statement;

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

async fn get_most_recent_mission(order_vec: Vec<kronos_order::Model>, db: &DatabaseConnection) -> Result<Option<String>, DbErr> {
    // Check all orders associated with this plan.
    // If there are any fragords, use the most recent. 
    // Else, check fro an OPORD.
    // Else, check fro a fragord and use the most recent.
    // Else, return NONE
    let mut champion: Option<&kronos_order::Model> = None;

    for order in &order_vec {
        champion = match champion {
            None => Some(order),
            Some(current_champion) => Some(more_recent_order(&current_champion, &order)),
        };
    }

    let mission: Option<String> = match champion {
        Some(order) => {
            let mission_paragraph = Paragraph::find()
                .filter(paragraph::Column::KronosOrder.eq(order.id))
                .filter(paragraph::Column::IndentLevel.eq(0)) // Major Paragraphs
                .filter(paragraph::Column::OrdinalSequence.eq(2)) // Paragraph 2 (mission)
                .one(db)
                .await?;
            let mission = match mission_paragraph {
                Some(paragraph) => Some(paragraph.text),
                None => None,
            };
            mission 
        },
        None=> None,
    };

    Ok(mission)
    
}    

fn more_recent_order<'a>(
    order_1: &'a kronos_order::Model, 
    order_2: &'a kronos_order::Model
) -> &'a kronos_order::Model {
    let order1rank = match order_1.order_type.as_str() {
        "FRAGORD" => 3,
        "OPORD" => 2,
        "WARNORD" => 1,
        _ => panic!("Invariant violated: Order had nonsense type {}", order_1.order_type), // Default case (optional)
    };
    let order2rank = match order_2.order_type.as_str() {
        "FRAGORD" => 3,
        "OPORD" => 2,
        "WARNORD" => 1,
        _ => panic!("Invariant violated: Order had nonsense type {}", order_2.order_type), // Default case (optional)
    };
    
    if order1rank == order2rank {
        // they are the same type of order. check serial numbers.
        if order_1.serial_number > order_2.serial_number {
            return order_1;
        }else if order_1.serial_number < order_2.serial_number {
            return order_2; 
        }else { 
            assert!(
                order_1.parent_plan == order_2.parent_plan, 
                "Invariant violated: Orders in order_vec not from same parent plan in get_plans return."
            );
            panic!("Invariant violated: Orders of same plan and same type had same serial numbers.");
        }
    }else{
        if order1rank > order2rank {
            return order_1;
        }else{
            return order_2;
        }
    }
}