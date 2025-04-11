//! delete_unit.rs

// This cascade deletes a unit (really, the database does this).
// Cleaning up the plans, the orders, etc.

use actix_web::web::Json;
use sea_orm::*;

use crate::{
    models::{entities::{
        prelude::*, 
        *
    }, entity_summaries::admin_summary::AdminSummary}, routes::api::parameters::{
        network_structs::*, 
        unit_request
    }, utilities::database_tools::{access_kronos_database, api_access_kronos_database}, *
};

pub async fn delete_unit(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    // To delete a unit, we need to look inside the request at the UIC (and nothing else)

    // In the future, this is a catastrophe if done on accident, so we need to ensure that 
    // proper controls are put on this. For now, leave it of the GUI so you'd need to either
    // have the source code or just be very patient with CURL.

    let unit_request = match &req.unit_request {
        Some(unit_request) => unit_request,
        None => return Err(KronosApiError::BadRequest("Requests to delete a unit must include a unit_request.".to_string())),
    };
    let target_uic = match &unit_request.uic {
        Some(target_uic) => target_uic,
        None => return Err(KronosApiError::BadRequest("Delete unit reqeusts must include a uic in teh unit_summary or deletion.".to_string())),
    };

    let db = api_access_kronos_database().await?;

    // We've acquired the target, delete it
    let unit = match Unit::find_by_id(target_uic.as_str()).one(&db).await {
        Ok(unit) => match unit {
            Some(unit) => unit,
            None => return Err(KronosApiError::ExpectedDataNotPresent(format!("There is no unit matching uic {}", target_uic)))
        },
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    let result: DeleteResult = match unit.delete(&db).await {
        Ok(result) => result,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    let kronos_response = wrap_delete_request(result, req)?;

    Ok(kronos_response)

}


fn wrap_delete_request(result: DeleteResult, req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    // I am making the bold assumption that if we affect more than 2147483647 rows we are in deep shit.
    let rows_affected = convert_u64_to_i32(result.rows_affected)?;
    
    let admin_summary : AdminSummary = AdminSummary {
         number_response: Some(rows_affected), 
         string_response: None, 
         rows_affected: None, 
    };
    let kronos_response: KronosResponse = KronosResponse { 
        kronos_request: req.into_inner(), 
        plans_vec: None, 
        orders_vec: None,
        paragraphs_vec: None,
        units_vec: None,
        tasks_vec: None,
        admin_vec: Some(vec![admin_summary]), 
    };
    Ok(kronos_response)
}

fn convert_u64_to_i32(target : u64) -> Result<i32, KronosApiError> {
    let result : i32 = match target.clone().try_into() {
        Ok(result) => result,
        Err(msg) => return Err(KronosApiError::InternalServerError(
            format!("The last operation (Delete) affected, somehow, at least 2^31 -1 rows. (2147483647). \
            This should never happen, or if it does, this software was wildly successful beyond anyone's imagining. 
            As a u64, it affected {} rows.
            Error message: {}", target.to_string(), msg.to_string())
        )),
    };
    Ok(result)
}