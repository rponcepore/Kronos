use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{Statement};

use crate::m20250316_000003_create_plan::Plan;
use crate::m20250317_000004_create_kronosorder::KronosOrder;
use crate::m20250317_000005_create_paragraph::Paragraph;
use crate::m20250316_000002_create_unit::Unit;

use crate::preloaded_data::fragord_data::*;
use crate::helper_methods::order_insertion::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        //First find the overall plan
        let plan_id = get_plan_id("WJH8AA", 25, 1, db, manager).await?;

        // Create an order associated with this plan.
        let mut order_vec: Vec<(&i32, &str, i32, bool)> = Vec::new();
        order_vec.push((&plan_id, "OPORD", 0, true)); 
        order_vec.push((&plan_id, "FRAGORD", 1, true)); 
        order_vec.push((&plan_id, "WARNORD", 1, true)); 
        

        for ord in order_vec{
        
            let order_id = insert_shallow_order(
                ord.0, // plan_id
                ord.1, // opord_type
                ord.2, // serial_number
                ord.3,
                db,
                manager
            ).await?;

            // For each order, add the first five paragraphs
            insert_header_paragraphs(order_id, manager);
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
