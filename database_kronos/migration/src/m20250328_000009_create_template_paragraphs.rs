use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{Statement};

use super::m20250316_000003_create_plan::Plan;
use super::m20250317_000004_create_kronosorder::KronosOrder;

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
        let fragord_params = (&plan_id, "FRAGORD");
        let opord_params = (&plan_id, "OPORD");
        let warnord = (&plan_id, "FRAGORD");

        let warnord_id = get_order_id(warnord_params, db, manager).await?;
        let opord_id = get_order_id(opord_params, db, manager).await?;
        let fragord_id = get_order_id(fragord_params, db, manager).await?;
        
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}

async fn get_order_id(params: (&i32, &str), db: &SchemaManagerConnection<'_>, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
    // Now get the fragord
    // Get the plan_id that we need
    let query = Statement::from_string(
        manager.get_database_backend(),
        format!(
            "SELECT id FROM {} WHERE parent_plan = '{}' AND order_type = {}",
            Plan::Table.to_string(),
            params.0,
            params.1 
        ),
    );

    let ord_id = match db.query_one(query).await? {
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

    Ok(ord_id)
}