//! order_insertion.rs
use sea_orm_migration::prelude::*;
use sea_orm::{Statement};

use crate::m20250316_000003_create_plan::Plan;
use crate::m20250317_000004_create_kronosorder::KronosOrder;
use crate::m20250317_000005_create_paragraph::Paragraph;

use crate::preloaded_data::order_templates::*;

// This file defines convenience methdos for accessing the database to do basic data entry on the migration side

// This function gets a plan id from teh databse given a unit, fiscal year, and serial number.
pub async fn get_plan_id(unit: &str, fiscal_year: i32, serial_number: i32, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
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

    let db = manager.get_connection();

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
pub async fn get_order_id(plan_id: &i32, order_type: &str, serial_number: Option<i32>, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
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

    let db = manager.get_connection();

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
pub async fn insert_shallow_order(plan_id: &i32, order_type: &str, serial_number: i32, is_published: bool, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
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

    let order_id = get_order_id(&plan_id, order_type, Some(serial_number), manager).await?;
    println!("Successfully inserted order pk = {}", order_id);
    Ok(order_id)
}

pub async fn insert_header_paragraphs(order_id: i32, manager: &SchemaManager<'_>) -> Result<(), DbErr> {

    let paragraphs = get_header_paragraph_vec();
    
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

/*
 * This method builds the standard template order. It builds from boilerplate to help the user fo complete coverage
 * of the subject matter.
 */
pub async fn build_standard_order(plan_id: &i32, manager: &SchemaManager<'_>) -> Result<i32, DbErr> {
    // Create an OPORD.
    let serial_number = 0;
    let order_type = "OPORD";
    let insert = Query::insert()
        .into_table(KronosOrder::Table)
        .columns([  KronosOrder::ParentPlan, 
                    KronosOrder::OrderType, 
                    KronosOrder::SerialNumber, 
                    KronosOrder::IsPublished])
        .values_panic([
            plan_id.clone().into(),
            order_type.into(),
            serial_number.into(),
            false.into(),
            ]) 
        .to_owned();

    manager.exec_stmt(insert).await?;
    
    // OPORD entry created. Retrieve the ID
    let order_id = get_order_id(&plan_id, order_type, Some(serial_number), manager).await?;

    // ID retrieved. Now build out its paragraphs. 
    let order_template: OrderTemplate = default_order_template();
    let mut ordinal_sequence: i32 = 1;
    let indent_level: i32 = 0;
    for major_paragraph in order_template.paragraphs{
        // insert major paragraph, returning paragraph id
        let _para_id = insert_paragraph(
            &order_id, 
            &ordinal_sequence, //1-5 for major paragraphs
            &indent_level,
            &major_paragraph,
            None,
            manager
            ).await?;
        
        // increment as necessary:
        ordinal_sequence +=1;
    }

    Ok(order_id)

}

// Recursive method that inserts a paragraph and all it's subpargraphs into the database.
// It is called at the top level by the major paragraphs
pub async fn insert_paragraph(
    order_id: &i32, 
    ordinal_sequence: &i32,
    indent_level: &i32,
    paragraph: &MigrationParagraph, 
    parent_paragraph_id: Option<&i32>,
    manager: &SchemaManager<'_> ) -> Result<i32, DbErr> {

        // Insert the paragraph into the db, returning the id for subparagraph reference
        let paragraph_id = insert_paragraph_into_db(
            order_id,
            ordinal_sequence,
            indent_level,
            paragraph,
            parent_paragraph_id, // Parent paragraph id
            manager,
        ).await?;

        // Recursively add the subparagraphs
        match &paragraph.subparagraphs {
            Some(subparagraphs) => {
                let sub_indent_level = indent_level + 1; // indent at start of loop, once
                let mut sub_ordinal_sequence = 1; //always start numbering at 1
                for subpara in subparagraphs {
                    // insert subparagraphs recursively, using the paragraph ID provided above for the first entry.
                    let _new_para_id = insert_paragraph(
                        order_id,
                        &sub_ordinal_sequence,
                        &sub_indent_level,
                        &subpara,
                        Some(&paragraph_id),
                        manager,
                    ).await?;
                    sub_ordinal_sequence += 1;
                }
            },
            None => {}, //Do nothing
        };

        Ok(paragraph_id)
    }

// Utility method for putting one and only one paragraph into the database
pub async fn insert_paragraph_into_db(
    order_id: &i32, 
    ordinal_sequence: &i32,
    indent_level: &i32,
    paragraph: &MigrationParagraph, 
    parent_paragraph_id: Option<&i32>, 
    manager: &SchemaManager<'_> ) -> Result<i32, DbErr> {
    
    let insert = match parent_paragraph_id {
        Some(parent_paragraph_id) => {
            Query::insert()
            .into_table(Paragraph::Table)
            .columns([  Paragraph::KronosOrder, 
                        Paragraph::OrdinalSequence, 
                        Paragraph::IndentLevel,
                        Paragraph::Title, 
                        Paragraph::Text,
                        Paragraph::ParentParagraph])
            .values_panic([
                order_id.clone().into(),
                ordinal_sequence.clone().into(),
                indent_level.clone().into(),
                paragraph.title.clone().into(),
                paragraph.text.clone().into(),
                parent_paragraph_id.clone().into()
                ]) 
            .to_owned()
        },
        None => {
            Query::insert()
            .into_table(Paragraph::Table)
            .columns([  Paragraph::KronosOrder, 
                        Paragraph::OrdinalSequence, 
                        Paragraph::IndentLevel,
                        Paragraph::Title, 
                        Paragraph::Text,])
            .values_panic([
                order_id.clone().into(),
                ordinal_sequence.clone().into(),
                indent_level.clone().into(),
                paragraph.title.into(),
                paragraph.text.into(),
                ]) 
            .to_owned()
        },
    };
    
    manager.exec_stmt(insert).await?;

    let db = manager.get_connection();

    /*
     * This query is a pain in the hoohah. We have to assume certain things about our paragraph--namely that there are no duplicate paragraph titles, etc. 
     */
    let query = match parent_paragraph_id {
        Some(parent_paragraph_id) => {
            Statement::from_string(
                manager.get_database_backend(),
                format!(
                    "SELECT id FROM {} WHERE kronos_order = {} AND ordinal_sequence = {} AND indent_level = {} AND parent_paragraph = {}",
                    Paragraph::Table.to_string(),
                    order_id,
                    ordinal_sequence,
                    indent_level,
                    parent_paragraph_id,
                ),
            )
        },
        None => {
            Statement::from_string(
                manager.get_database_backend(),
                format!(
                    "SELECT id FROM {} WHERE kronos_order = {} AND ordinal_sequence = {} AND indent_level = {}",
                    Paragraph::Table.to_string(),
                    order_id,
                    ordinal_sequence,
                    indent_level,
                ),
            )
        },
    };

    let para_id_vec = db.query_all(query).await?;
    
    let para_id:i32 = match para_id_vec.len() {
        0 => {
            return Err(sea_orm_migration::DbErr::RecordNotFound("No matching record found.".to_string()));
            },
        1 => para_id_vec[0]
            .try_get::<i32>("id", "id")? // Extract the ID and propagate any error
            .to_owned(),
        _ => {
            return Err(sea_orm_migration::DbErr::Custom("Multiple records found for what should be a unique query".to_string()));
        }
    };
    
    Ok(para_id)
    
}