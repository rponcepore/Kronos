use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{Statement};

use super::m20250316_000003_create_plan::Plan;
use super::m20250317_000004_create_kronosorder::KronosOrder;
use super::m20250317_000005_create_paragraph::Paragraph;
use crate::m20250316_000002_create_unit::Unit;

use super::preloaded_data::fragord_data::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        
        //First find the overall plan
        let plan_params: (&str, i32, i32) = ("WJH8AA", 25, 1);

        let plan_id = match get_plan_id(&plan_params, db, manager)?;

        // Create an order associated with this plan.
        let mut order_vec: Vec<(i32, &str, i32, bool)> = Vec::new();
        order_vec.push((plan_id, "OPORD", 0, true)); 
        order_vec.push((plan_id, "FRAGORD", 1, true)); 
        order_vec.push((plan_id, "WARNORD", 1, true)); 
        

        for fragord in order_vec{
            let insert = Query::insert()
                .into_table(KronosOrder::Table)
                .columns([  KronosOrder::ParentPlan, 
                            KronosOrder::OrderType, 
                            KronosOrder::SerialNumber, 
                            KronosOrder::IsPublished])
                .values_panic([
                    fragord.0.into(),
                    fragord.1.into(),
                    fragord.2.into(),
                    fragord.3.into(),
                    ]) 
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.exec_stmt(
            Query::delete()
                .from_table(KronosOrder::Table)
                .cond_where(Expr::col(Unit::Uic).is_in(["WJH8AA"]))
                .to_owned()
        ).await?;

        Ok(())
    }
}
