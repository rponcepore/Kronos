//!delete_paragraph.rs

use actix_web::web::Json;
use sea_orm::*;
use debug_print::debug_println as dprintln;

use crate::routes::api::parameters::{network_structs::*, paragraph_request};
use crate::utilities::database_tools::access_kronos_database;

use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::paragraph_summary::ParagraphSummary;

use crate::routes::api::helper_methods::assemble_paragraph_summary::*;


pub async fn delete_paragraph(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    todo!()
}