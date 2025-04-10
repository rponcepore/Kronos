//! test_utilities.rs

// This file provides necessary support for the testing packages.
use backend_kronos::routes::api::parameters::network_structs::KronosRequest;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

const RANDOM_PORT: &str = "127.0.0.1:0"; // This is a special case: says, "use local host, but bind port 0--any random port."

/* Spawn application in the background as a helper function */
pub fn spawn_app() -> String {
    let listener = TcpListener::bind(RANDOM_PORT).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = backend_kronos::startup::run_server(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    // return the port to the calling function, so the test goes to the correct port!
    format!("http://127.0.0.1:{}", port)
}

// This method builds a basic KronosRequest for when the database is not connected.
pub fn no_db_plan_req() -> KronosRequest {
    KronosRequest {
        api_method: Some("get_plans".to_string()),
        uic: Some("tstUIC".to_string()),
        admin_request: None,
        unit_request: None,
        plan_request: None,
        order_request: None,
        paragraph_request: None,
        task_request: None,

    }
}

// This method builds a basic KronosRequest for when the database is connected.
pub fn with_db_plan_req() -> KronosRequest {
    KronosRequest {
        api_method: Some("get_plans".to_string()),
        uic: Some("WJH8C0".to_string()),
        admin_request: None,
        unit_request: None,
        
        plan_request: None,
        order_request: None,
        paragraph_request: None,
        task_request: None,
    }
}

/* Return a dummy JSON for a post request asking for all plans */
pub fn dummy_plan_request() -> KronosRequest {
    let result = KronosRequest {
        api_method: Some("get_plans".to_string()),
        uic: Some("WHJ8C0".to_string()),
        admin_request: None,
        unit_request: None,
        
        plan_request: None,
        order_request: None,
        paragraph_request: None,
        task_request: None,
    };
    result
}
