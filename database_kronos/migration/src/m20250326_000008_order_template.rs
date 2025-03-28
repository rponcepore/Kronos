use sea_orm::{prelude::*, ExecResult, DatabaseBackend};
use sea_orm::{Statement};
use sea_orm_migration::{prelude::*};

use sea_orm_migration::DbErr;

// Bring plans table into scope
use super::m20250316_000003_create_plan::Plan;
use super::m20250317_000004_create_order::KronosOrder;

// Be able to connect to the database, this is awful.
// use backend_kronos::routes::api::api_handler::access_kronos_database;


/* This migration conducts a series of insert queries to seed the database with some basic templates. */

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // This stores a template for an order. This is a very bad hack at this time,
        // As we're just calling all orders under the UIC "TEMPLT" and moving on

        
        
        //First create the overall plan "00-00 Orders Template"
        let plan: (&str, i32, i32, &str, &str) = ("TEMPLT", 00, 0, "Orders Templates", "CUI");

        let insert = Query::insert()
            .into_table(Plan::Table)
            .columns([  
                Plan::Unit, 
                Plan::FiscalYear, 
                Plan::SerialNumber, 
                Plan::Name, 
                Plan::Classification])
            .values_panic([
                plan.0.into(),
                plan.1.into(),
                plan.2.into(),
                plan.3.into(),
                plan.4.into(),
            ]) 
            .to_owned();

        manager.exec_stmt(insert).await?;
        
        /*
            This is a somewhat bad hack. SeaORM does not allow you to get the id of the last inserted query in the 
            SeaORM migrator framework, which is a separate crate from SeaORM. So I have to import some of the SeaORM
            shit, and it's dependencies, and the types, from my src directory, in order to run a query on the database,
            which will in turn allow me to find the primary key of the thing that I JUST INSERTED so that I can now make 
            some related records. The alternative would be to make synthetic, natural primary keys, so that I could logically
            call for a primary key from the database, at least for plans, but it would bloat the field. i.e., "WJH8A0202501" would
            be the key for UIC WJH8A0 FY 2025 SN 1, which should never be duplicated.

            Insult to injury, we have to use raw SQL anyway, because to use teh Sea_ROM functionality we woul dneed to point 
            back to our entities, which would introduce circular dependencies at build time or require custom bash scripting
            to start the app up correctly. 

            I considered changing everything to natural keys, but then I would need to go change all our frontend types. 
            I could put a checking method into startup, i.e., if the database is available, check if the basic templates 
            were loaded, and if they weren't, then load them direectly from the application. But given that the application 
            doesn't have any data built into at the moment, it would start to un-separate concerns, which I don't want to do. 
            So raw sql it is.
         */

        // Now get that plan ID. First we have to connect to the database.

        let db = manager.get_connection();
        
        // Query the last inserted row (assuming `id` is the primary key)
        let query = Statement::from_string(
            manager.get_database_backend(),
            format!(
                "SELECT id FROM {} WHERE unit = '{}' AND fiscal_year = {} AND serial_number = {}",
                Plan::Table.to_string(),
                plan.0, // "TEMPLT"
                plan.1, // 0
                plan.2 // 0 
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

        // Now create the templates

        /*
        pub enum KronosOrder {
            Table,
            Id, //auto
            ParentPlan, //""
            OrderType,
            SerialNumber,
            IsPublished,
            DerivedFrom,
        }
        */

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
        // This should cascade delete all templates, joy.
        manager.exec_stmt(
            Query::delete()
                .from_table(Plan::Table)
                .cond_where(Expr::col(Plan::Name).is_in(["TEMPLT"]))
                .to_owned()
        ).await?;
        Ok(())
    }
}


