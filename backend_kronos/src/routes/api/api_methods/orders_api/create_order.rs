//! create_order.rs

// This file defines the internal actions for creating a new order and populating it to the database.

use actix_web::web::Json;

use crate::routes::api::parameters::network_structs::*;



pub async fn create_order(req: Json<KronosRequest>) -> Result<KronosResponse, KronosApiError> {
    todo!()
}