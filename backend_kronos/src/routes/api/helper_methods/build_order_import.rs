//! order_builder.rs
//! 

use serde::{Serialize, Deserialize};
use std::fs;
use crate::models::entities::{
    plan,
    kronos_order,
    paragraph
};
use crate::models::entity_summaries::{
    plan_summary::*,
    kronos_order_summary::*,
    paragraph_summary::*,
};
use crate::routes::api::parameters::network_structs::KronosApiError;

const PATH_TO_OPORD_FILE: &str = "../../../../../configs/standard_opord_contents.yaml";

/* TYPES
pub struct KronosOrderSummary {
    pub data: KronosOrder,
    pub paragraphs: Option<Vec<ParagraphSummary>>,
}
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "kronos_order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub parent_plan: i32,
    pub order_type: String,
    pub serial_number: i32,
    pub is_published: bool,
    pub derived_from: Option<i32>,
}

pub struct ParagraphSummary {
    pub data: Paragraph,
    pub subparagraphs: Option<Vec<ParagraphSummary>>,
}
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "paragraph")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub kronos_order: i32,
    pub parent_paragraph: Option<i32>,
    pub ordinal_sequence: i32,
    pub title: String,
    pub text: String,
    pub indent_level: i32,
}
*/

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportOrder{
    paragraphs: Vec<ImportParagraph>
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportParagraph{
    ordinal_sequence: i32,
    title: String,
    text: String,
    subparagraphs: Vec<ImportParagraph>
}

pub fn make_standard_order() -> Result<ImportOrder, KronosApiError> {
    let yaml_str = match fs::read_to_string(PATH_TO_OPORD_FILE){
        Ok(yaml_str) => yaml_str,
        Err(msg) => return Err(KronosApiError::Unknown(format!("Failed to read file at {}", PATH_TO_OPORD_FILE))),
    };
    let opord: ImportOrder = match serde_yaml::from_str(&yaml_str) { // crate deprecated but unlikely to change soon.
        Ok(opord) => opord,
        Err(msg) => return Err(KronosApiError::Unknown(format!("Failed to serialize string to ImportOpord. 7str: {}", yaml_str))),
    };

    println!("{:#?}", opord);
    Ok(opord)
}