//! order_insertion.rs
use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{Statement};

use crate::m20250316_000003_create_plan::Plan;
use crate::m20250317_000004_create_kronosorder::KronosOrder;
use crate::m20250317_000005_create_paragraph::Paragraph;

use crate::preloaded_data::fragord_data::*;

// This file defines convenience methdos for accessing the database to do basic data entry on the migration side

// This function gets a plan id from teh databse given a unit, fiscal year, and serial number.
pub async fn get_plan_id(params: (&str, i32, i32), db: &SchemaManagerConnection<'_>, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
    //First find the overall plan
    let plan = params;

    // Query the last inserted row (assuming `id` is the primary key)
    let query = Statement::from_string(
        manager.get_database_backend(),
        format!(
            "SELECT id FROM {} WHERE unit = '{}' AND fiscal_year = {} AND serial_number = {}",
            Plan::Table.to_string(),
            plan.0, // "WJH8AA"
            plan.1, // 25
            plan.2 // 1
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
pub async fn get_order_id(params: (&i32, &str, Option<i32>), db: &SchemaManagerConnection<'_>, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
    // Now get the fragord
    // Get the plan_id that we need

    //Case where we have an order serial number.
    let query = match params.2 {
        Some(serial_number) => Statement::from_string(
            manager.get_database_backend(),
            format!(
                "SELECT id FROM {} WHERE parent_plan = {} AND order_type = '{}' AND serial_number = {}",
                KronosOrder::Table.to_string(),
                params.0,
                params.1,
                serial_number
            ),
        ),
        None => Statement::from_string(
            manager.get_database_backend(),
            format!(
                "SELECT id FROM {} WHERE parent_plan = {} AND order_type = '{}'",
                KronosOrder::Table.to_string(),
                params.0,
                params.1
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

pub async fn insert_shallow_order() -> Result<i32, DbErr> {

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