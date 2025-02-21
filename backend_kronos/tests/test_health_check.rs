//! tests/health_check.rs

use std::net::TcpListener;

use backend_kronos::configuration::{self, get_configuration};

const RANDOM_PORT: &str = "127.0.0.1:0";

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
async fn database_health_check () {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    
    // Now attempt to connect to the database.

}



/* Spawn application in the background as a helper function */
fn spawn_app() -> String {
    let listener = TcpListener::bind(RANDOM_PORT)
    .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = backend_kronos::run_server(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    // return the port to the calling function, so the test goes to the correct port!
    format! ("http://127.0.0.1:{}", port)
}