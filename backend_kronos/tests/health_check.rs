//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using 
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works () {
    //Arrange
    spawn_app();
    //reqwest is a library tha performs HTTP requests against our application.
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert! (response.status().is_success());
    assert_eq! (Some(0), response.content_length() );
}

/* Spawn application in the background as a helper function */
fn spawn_app() {
    let server = backend_kronos::run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}