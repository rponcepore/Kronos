//! admin_request_handler.rs

use actix_web::web::Json;
use sea_orm::*;

use crate::{routes::api::parameters::network_structs::{
    KronosApiError, KronosRequest, KronosResponse,
}, utilities::database_tools::access_kronos_database};

use crate::models::entity_summaries::admin_summary::*;
use crate::models::entities::{prelude::*, *};

pub async fn admin_request_handler(
    req: Json<KronosRequest>,
) -> Result<KronosResponse, KronosApiError> {


    // What's the question?

    // Send appropriate response

    // start database
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    // for now, just send the number of units in the database
    let selection = Unit::find();
    let count_u64 = match selection.count(&db).await {
        Ok(count_u64) => count_u64,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };
    let count_i32: i32 = count_u64.try_into().map_err(|_| {
        KronosApiError::InternalServerError("Too many units in database to fit in i32".to_string())
    })?;

    let admin = AdminSummary {
        number_response: Some(count_i32),
        string_response: None,
    };

    let kronos_response = KronosResponse::new(req).with_admin(admin);

    Ok(kronos_response)
}
