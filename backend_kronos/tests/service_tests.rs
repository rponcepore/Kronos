//! network_tests.rs

// This file defines all tests written that create an instance of the app
// and test against it, rather than local tests, which only test some internal
// functionality.
use serde_json::json;
use serde::{Serialize, Deserialize};

mod test_utilities;

use test_utilities::*;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using 
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works () {
    //Arrange
    let address = spawn_app();
    //reqwest is a library tha performs HTTP requests against our application.
    let client = reqwest::Client::new();

    //Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn api_returns_400_for_blank_post_plan_request () {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Extract response status and body
    let status = &response.status();
    let body = &response.text().await.expect("Failed to read response body");

    assert!(
        &status.is_client_error(),
        "Request failed! Status: {}, Response Body: {}",
        status,
        body
    );

}

#[tokio::test]
async fn api_returns_200_for_any_post_plan_request () {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let dummy_plan_request = dummy_plan_request();

    let response = client
        .post(&format!("{}/api", &address))
        .json(&dummy_plan_request)
        .send()
        .await
        .expect("Failed to execute request.");
    
    
    // Extract response status and body
    let status = &response.status();
    let body = &response.text().await.expect("Failed to read response body");

    assert!(
        &status.is_success(),
        "Request failed! Status: {}, Response Body: {}",
        status,
        body
    );
}