//! order_insertion.rs
use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{Statement};

use crate::m20250316_000003_create_plan::Plan;
use crate::m20250317_000004_create_kronosorder::KronosOrder;
use crate::m20250317_000005_create_paragraph::Paragraph;

use crate::preloaded_data::fragord_data::*;

// This file defines convenience methdos for accessing the database to do basic data entry on the migration side

// This function gets a plan id from teh databse given a unit, fiscal year, and serial number.
pub async fn get_plan_id(unit: &str, fiscal_year: i32, serial_number: i32, db: &SchemaManagerConnection<'_>, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
    //First find the overall plan

    // Query the last inserted row (assuming `id` is the primary key)
    let query = Statement::from_string(
        manager.get_database_backend(),
        format!(
            "SELECT id FROM {} WHERE unit = '{}' AND fiscal_year = {} AND serial_number = {}",
            Plan::Table.to_string(),
            unit, // "WJH8AA"
            fiscal_year, // 25
            serial_number // 1
        ),
    );

    let plan_id = match db.query_one(query).await? {
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

    Ok(plan_id)
}

// This function gets an order form the database with the plan id and order type, and possibly the order serial number.
pub async fn get_order_id(plan_id: &i32, order_type: &str, serial_number: Option<i32>, db: &SchemaManagerConnection<'_>, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
    // Now get the fragord
    // Get the plan_id that we need

    //Case where we have an order serial number.
    let query = match serial_number {
        Some(serial_number) => Statement::from_string(
            manager.get_database_backend(),
            format!(
                "SELECT id FROM {} WHERE parent_plan = {} AND order_type = '{}' AND serial_number = {}",
                KronosOrder::Table.to_string(),
                plan_id,
                order_type,
                serial_number
            ),
        ),
        None => Statement::from_string(
            manager.get_database_backend(),
            format!(
                "SELECT id FROM {} WHERE parent_plan = {} AND order_type = '{}'",
                KronosOrder::Table.to_string(),
                plan_id,
                order_type
            ),
        ),
    };

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

// This function takes a plan ID and inserts an order into the plan. 
// It returns the order ID that was just inserted.
pub async fn insert_shallow_order(plan_id: &i32, order_type: &str, serial_number: i32, is_published: bool, db: &SchemaManagerConnection<'_>, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
    println!("Attempting to insert orders into table for plan id #{}, order_type: {}, serno: {}.", plan_id, order_type, serial_number);

    let plan_id_clone = plan_id.clone();

    let insert = Query::insert()
                .into_table(KronosOrder::Table)
                .columns([  KronosOrder::ParentPlan, 
                            KronosOrder::OrderType, 
                            KronosOrder::SerialNumber, 
                            KronosOrder::IsPublished])
                .values_panic([
                    plan_id_clone.into(),
                    order_type.into(),
                    serial_number.into(),
                    is_published.into(),
                    ]) 
                .to_owned();

    manager.exec_stmt(insert).await?;

    let order_id = get_order_id(&plan_id, order_type, Some(serial_number), db, manager).await?;
    println!("Successfully inserted order pk = {}", order_id);
    Ok(order_id)
}

pub async fn insert_paragraphs_to_order_shallow(order_id: i32, 
                                            paragraphs: Vec<(i32, i32, &'static str, &'static str)>, 
                                            manager: &SchemaManager<'_>
                                            ) -> Result<(), DbErr> {
    
    /*
    pub enum Paragraph {
        Table,
        Id,
        KronosOrder,
        ParentParagraph,
        IsMajor,
        OrdinalSequence,
        Title,
        Text,
        IndentLevel, // 0 is base for paragraphs 1-5.
    }
     */
    
    // Paragraph: (i32, i32, &str, &str)
    for paragraph in paragraphs {
        let insert = Query::insert()
            .into_table(Paragraph::Table)
            .columns([
                Paragraph::IndentLevel,
                Paragraph::OrdinalSequence, 
                Paragraph::Title, 
                Paragraph::Text, 
                Paragraph::KronosOrder,
                ])
            .values_panic([
                paragraph.0.into(),
                paragraph.1.into(),
                paragraph.2.into(),
                paragraph.3.into(),
                order_id.into(),
            ]) 
            .to_owned();

        manager.exec_stmt(insert).await?;
    }
    Ok(())
}