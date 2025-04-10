//! build_unit_summary.rs

use sea_orm::*;

use crate::{
    models::{
        entities::{
            prelude::*,
            unit::*, 
            plan::*,
            *
        },
        entity_summaries::{
            unit_summary::UnitSummary,
            plan_summary::PlanSummary,
        }
    },
    routes::api::{
        parameters::network_structs::KronosApiError,
        helper_methods::summarizers::*,
    }
};

// Shallow. Just get shallow plan summaries
pub async fn build_unit_summary(unit: &unit::Model, db: &DatabaseConnection) -> Result<UnitSummary, KronosApiError> {
    // Get all the plans for that unit
    let plan_vec: Vec<plan::Model> = match Plan::find()
        .filter(plan::Column::Uic.contains(unit.uic.to_owned()))
        .order_by_asc(plan::Column::FiscalYear)
        .order_by_desc(plan::Column::SerialNumber)
        .all(db)
        .await
    {
        Ok(plan_vec) => plan_vec,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    let mut plan_summary_vec: Vec<PlanSummary> = Vec::new();
    // For each plan returned, get it's associated orders
    for plan in plan_vec {
        let packed_plan_summary = match pack_plan_summary_shallow(plan, &db).await {
            Ok(packed_plan_summary) => packed_plan_summary,
            Err(err) => return Err(err),
        };
        plan_summary_vec.push(packed_plan_summary);
    }

    let unit_summary = UnitSummary {
        data: unit.clone(),
        plans: Some(plan_summary_vec),
    };

    Ok(unit_summary)
}