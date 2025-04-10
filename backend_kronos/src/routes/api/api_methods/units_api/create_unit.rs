//! create_unit.rs

// This file defines the actions taken for creating a unit in the database.

use actix_web::web::Json;
use sea_orm::*;

// Network utilities
use crate::routes::api::parameters::network_structs::*;
use crate::routes::api::parameters::unit_request::UnitRequest;
use crate::utilities::database_tools::access_kronos_database;

//Entities
use crate::models::entities::{prelude::*, *};
use crate::models::entity_summaries::unit_summary::UnitSummary;

//Utility functions
use crate::routes::api::helper_methods::{build_unit_summary::*, uic_helpers::*};
use crate::reference::army_echelons_enum::Echelon;

struct ValidUnitRequest {
    pub uic: String,
    pub echelon : Echelon,
    pub nickname : String,
    pub display_name : String,
    pub short_name : String,
    pub component : String,
    pub state_abbrev : Option<String>,
    pub level : Option<i32>,
    pub service_member_capacity : Option<i32>,
    pub parent_uic: Option<String>,
}

pub async fn create_plan(valid_req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {

    // Check that this request has the unit parameters correctly.
    // A create unit request shoul dhave an entry for unit_request and within that a uic to be created.

    // Then, deduce the serial number (database call)
    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    let unit_request = match &valid_req.unit_request {
        Some(unit_request) => unit_request,
        None => return Err(KronosApiError::BadRequest("Request to create new unit did not include a valid unit_request object/field.".to_string())),
    };

    let checked_unit_request = check_valid_create_unit_request(&unit_request).await?;
    let unit_summary = add_unit_to_db(&checked_unit_request, &db).await?;
    let kronos_response = KronosResponse::new(valid_req).with_unit(unit_summary);

    Ok(kronos_response)
}



async fn add_unit_to_db(unit_request: &ValidUnitRequest, db: &DatabaseConnection) -> Result<UnitSummary, KronosApiError> {

    let new_unit = unit::ActiveModel {
        uic: ActiveValue::Set(unit_request.uic.to_owned()),
        echelon : ActiveValue::Set(unit_request.echelon.as_str().to_owned()),
        nickname : ActiveValue::Set(unit_request.nickname.to_owned()),
        display_name : ActiveValue::Set(unit_request.display_name.to_owned()),
        short_name : ActiveValue::Set(unit_request.short_name.to_owned()),
        component :ActiveValue::Set(unit_request.component.to_owned()),
        state_abbrev :ActiveValue::Set(unit_request.state_abbrev.to_owned()),
        level : ActiveValue::Set(unit_request.level.to_owned()),
        service_member_capacity : ActiveValue::Set(unit_request.service_member_capacity.to_owned()),
        parent_uic : ActiveValue::Set(unit_request.parent_uic.to_owned()),

    };

    let unit: unit::Model = match new_unit.insert(db).await {
        Ok(result) => result,
        Err(msg) => return Err(KronosApiError::DbErr(msg)),
    };

    let unit_summary = build_unit_summary(&unit, db).await?;

    Ok(unit_summary)
}


async fn check_valid_create_unit_request(req: &UnitRequest) -> Result<ValidUnitRequest, KronosApiError> {
    let uic = match req.uic.clone() {
        Some(uic) => uic,
        None => return Err(KronosApiError::BadRequest("Request to create a new unit did not include a valid uic in the unit_request object.".to_string())),
    };

    // Make sure that this UIC only has six characters, unless the first five are "test"
    // in which case limit it to 80 characters.
    let _ = check_uic_length(&uic);

    let db = match access_kronos_database().await {
        Ok(db) => db,
        Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
    };

    // Make sure that this UIC doesn't already exist.
    let possible_uic = match Unit::find_by_id(&uic)
        .one(&db)
        .await {
            Ok(possible_uic) => match possible_uic{
                Some(possible_uic) => {
                    return Err(KronosApiError::BadRequest(format!("The UIC {} already exists.", uic)));
                },
                None => {}, //base case
            },
            Err(db_err) => return Err(KronosApiError::DbErr(db_err)),
        };
    
    /*
        let unit_summary = match req.parent_uic {
            Some(parent_uic) => {
                check_valid_uic(parent_uic)?;
            },
            None => {
                //allow in testing only, until we start getting well-seeded data.
            }, 
        };
     */

    

    // Everything is validated. Assemble the ValidUnitRequest
    let mut valid_unit_request = ValidUnitRequest::new(&uic);

    // Now just do some basic validation
    /*
    // uic: String,
    // echelon : Echelon, // Echelon is already checked via the enum
    // nickname : String, 
    // display_name : String,
    // short_name : String,
    // component : String,
    state_abbrev : Option<String>,
    level : Option<String>,
    service_member_capacity : Option<i32>,
    // parent_uic: Option<String>,
    */
    
    valid_unit_request.nickname = require_non_empty_field(&req.nickname, "nickname")?;
    valid_unit_request.display_name = require_non_empty_field(&req.display_name, "display_name")?;
    valid_unit_request.short_name = require_non_empty_field(&req.short_name, "short_name")?;
    valid_unit_request.component = require_non_empty_field(&req.component, "component")?;

    //check the rest of component
    match &req.component {
        Some(component) => match component.as_str() {
            "ACTIVE" => {},
            "RESERVE" => {},
            "GUARD" =>  {},
            _ => return Err(KronosApiError::BadRequest("Component must be either ACTIVE, RESERVE, or GUARD.".to_string())),
        },
        None => return Err(KronosApiError::BadRequest("Component is required.".to_string())),
    };
    
    valid_unit_request.state_abbrev = req.state_abbrev.clone();
    valid_unit_request.level = req.level.clone();
    valid_unit_request.service_member_capacity = req.service_member_capacity.clone();
    
    // Now check the parent uic, if provided:
    if let Some(parent_uic) = &req.parent_uic {
        let _ = check_valid_uic(parent_uic, &db).await?;
    };

    valid_unit_request.parent_uic = req.parent_uic.clone();

    Ok(valid_unit_request)

}

// fn check_valid_create_unit_request()

impl ValidUnitRequest {
    pub fn new(uic: &String) -> Self {
        Self {
            uic: uic.clone(),
            echelon: Echelon::UNK,
            nickname: String::new(),
            display_name: String::new(),
            short_name: String::new(),
            component: String::new(),
            state_abbrev: None,
            level: None,
            service_member_capacity: None,
            parent_uic: None,
        }
    }
}

// Helper function to reduce code repetition
fn require_non_empty_field(value: &Option<String>, field_name: &str) -> Result<String, KronosApiError> {
    
    let stringval = match value {
        Some(v) if v.trim().is_empty() => return Err(KronosApiError::BadRequest(format!("{field_name} cannot be an empty string."))),
        Some(stringval) => stringval,
        None => return Err(KronosApiError::BadRequest(format!("{field_name} cannot be null. You must provide a value."))),
    };

    Ok(stringval.to_owned())
}