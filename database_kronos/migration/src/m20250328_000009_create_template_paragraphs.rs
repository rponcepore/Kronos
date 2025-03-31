use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{Statement};

use super::m20250316_000003_create_plan::Plan;
use super::m20250317_000004_create_kronosorder::KronosOrder;
use super::m20250317_000005_create_paragraph::Paragraph;

use super::preloaded_data::fragord_data::*;
use super::helper_methods::order_insertion::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db: &SchemaManagerConnection = manager.get_connection();

        // First, identify which order we want to connect these to. It should be the 
        // only plan associated with unit "TEMPLT", fy 0 sn 0
        let parent_plan_name = "Orders Template Plan";
        
        // Get the plan_id that we need
        let query = Statement::from_string(
            manager.get_database_backend(),
            format!(
                "SELECT id FROM {} WHERE name = '{}'",
                Plan::Table.to_string(),
                parent_plan_name 
            ),
        );

        let plan_id= match db.query_one(query).await? {
            Some(row) => {
                let id: i32 = row.try_get("", "id")?;
                println!("Found ID: {}", id);
                id
            }
            None => {
                println!("No matching record found.");
                return Err(sea_orm_migration::DbErr::RecordNotFound("No matching record found.".to_string()));
            }
        };

        

        // There are three combination: 

        let warnord_id = get_order_id(&plan_id, "WARNORD", None::<i32>, db, manager).await?;
        let opord_id = get_order_id(&plan_id, "OPORD", None::<i32>, db, manager).await?;
        let fragord_id = get_order_id(&plan_id, "FRAGORD", None::<i32>, db, manager).await?;
        
        // Insert warnord's paragraphs
        //insert_paragraphs_to_order_shallow(warnord_id, wa)

        // OPORD

        // FRAGORD
        let fragord_paragraphs = get_fragord_vec();
        insert_paragraphs_to_order_shallow(fragord_id, fragord_paragraphs, manager).await?;


        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db: &SchemaManagerConnection = manager.get_connection();

        let plan_id = get_plan_id(("TEMPLT", 0,0), db, manager).await?;

        
        // Replace the sample below with your own migration scripts
        let warnord_id = get_order_id(&plan_id, "WARNORD", None::<i32>, db, manager).await?;
        let opord_id = get_order_id(&plan_id, "OPORD", None::<i32>, db, manager).await?;
        let fragord_id = get_order_id(&plan_id, "FRAGORD", None::<i32>, db, manager).await?;

        manager.exec_stmt(
            Query::delete()
                .from_table(Paragraph::Table)
                .cond_where(Expr::col(Paragraph::KronosOrder).is_in([warnord_id, opord_id, fragord_id]))
                .to_owned()
        ).await?;

        Ok(())

    }
}

