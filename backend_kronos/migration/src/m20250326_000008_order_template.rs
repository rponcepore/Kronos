use sea_orm_migration::{prelude::*, schema::*};

// Bring plans table into scope
use super::m20250316_000003_create_plan::Plan;
use super::m20250317_000004_create_order::KronosOrder;


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
            .columns([  Plan::Unit, 
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

        // Now get that plan ID
        let plan: Option<i32> = Plan::find()
            .filter(plan::Column::Unit.eq("TEMPLT"))
            .filter(plan::Column::FiscalYear.eq(0))
            .filter(plan::Column::SerialNumber.eq(0))
            .one(manager.get_connection())
            .await?;
        
        let plan_id = plan.unwrap().id;
        
        
        // Now create the order "WARNORD_TEMPLATE"

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
        order_vec.push((plan_id, "OPORD", 0, true)); //
        

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


