//! integration_tests.rs

// These tests assume that there is a functional database, and that it is available for connections.
mod test_utilities;

use test_utilities::*;

#[tokio::test]
async fn api_returns_200_for_a_valid_no_database_post_plan_request() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let dummy_plan_request = no_db_plan_req();

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

#[tokio::test]
async fn api_returns_200_for_a_valid_open_database_post_plan_request() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let dummy_plan_request = with_db_plan_req();

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
