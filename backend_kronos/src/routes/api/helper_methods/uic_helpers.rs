//! uic_helpers.rs

use sea_orm::*;

use crate::routes::api::parameters::network_structs::KronosApiError;
use crate::models::entities::{prelude::*, unit::*};


pub async fn check_valid_uic(uic: &String, db: &DatabaseConnection) -> Result<(), KronosApiError> {
    
    check_uic_length(uic)?;

    if check_if_uic_exists(uic, db).await?{
        // base case, i.e., it exists and threw no errors
    }else{
        return Err(KronosApiError::BadRequest("The requested UIC does not exist.".to_string()));
    }
    Ok(())
}

pub async fn check_if_uic_exists(uic: &String, db: &DatabaseConnection) -> Result<bool, KronosApiError> {
    let unit = match Unit::find_by_id(uic)
        .one(db)
        .await {
        Ok(unit) => unit,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };
    let result = match unit {
        Some(unit) => true,
        None => false,
    };
    Ok(result)
}

pub fn check_uic_length(uic : &String) -> Result<(), KronosApiError> {
    if uic.len() != 6 {
        //check if it's a test
        if uic.starts_with("test") {
            if uic.len() > 80 {
                return Err(KronosApiError::BadRequest("UIC must either be 6 characters long, or start with 'test' and be no longer than 80 characters.".to_string()));
            }
        }else{
            // not a test UIC
            return Err(KronosApiError::BadRequest("UIC must either be 6 characters long, or start with 'test' and be no longer than 80 characters.".to_string()));
        }
    }
    Ok(())
}