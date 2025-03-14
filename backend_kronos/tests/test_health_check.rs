//! tests/health_check.rs

use std::net::TcpListener;
use sea_orm::*;


use backend_kronos::entities::{prelude::*, *};
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
async fn database_alive_test () {
    // let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    
    // Now attempt to connect to the database. 
    match Database::connect(connection_string).await {
        Ok(..) => println!("Connection successful."),
        Err(e) => panic!("Failed to connect to database: {}", e),
    };
}

#[tokio::test]
async fn database_crud_test () {
    // let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    
    // Now attempt to connect to the database. 
    let connection = Database::connect(connection_string).await.expect("Failed to connect to database.");

    // First, clean the table:
    let test_records : Vec<test_table::Model> = match TestTable::find().all(&connection).await {
        Ok(test_records) => test_records,
        Err(e) => panic!("{}", e),
    };

    // Delete each record from the vector
    for record in test_records {
        let _ = match record.delete(&connection).await {
                Ok(_) => {},
                Err(e) => panic!("{}", e),
            };
    }

    // We'll insert and retrieve a test record, then delete the record.
    let test_record = test_table::ActiveModel {
        id: ActiveValue::Set(0),
        title: ActiveValue::Set("Test_Name".to_owned()), //to_owned converts &str to String
        text: ActiveValue::Set("Test_Text".to_owned()),
    };

    match TestTable::insert(test_record).exec(&connection).await {
        Ok(_) => {},
        Err(e) => panic!("{}", e), 
    }; 

    // Find by the id
    let test_record_single : Option<test_table::Model> = match TestTable::find_by_id(0).one(&connection).await {
        Ok(test_record_single) => test_record_single,
        Err(e) => panic!("{}", e),
    };

    assert_eq!(test_record_single.unwrap().id, 0);

    // Find the record by filter matching
    let test_record_filter_one : Option<test_table::Model> = match TestTable::find()
        .filter(test_table::Column::Text.eq("Test_Text"))
        .one(&connection)
        .await {
            Ok(test_record_filter_one) => test_record_filter_one,
            Err(e) => panic!("{}", e),
        };


    assert_eq!(test_record_filter_one.unwrap().id, 0);

    // Finally, delete the entry (and any duplicate others, too)
    let test_records : Vec<test_table::Model> = match TestTable::find().all(&connection).await {
        Ok(test_records) => test_records,
        Err(e) => panic!("{}", e),
    };

    // Delete each record from the vector
    for record in test_records {
        let _ = match record.delete(&connection).await {
                Ok(_) => {},
                Err(e) => panic!("{}", e),
            };
    }

    // Ensure that they were deleted
    let test_records : Vec<test_table::Model> = match TestTable::find().all(&connection).await {
        Ok(test_records) => test_records,
        Err(e) => panic!("{}", e),
    };

    assert!(test_records.is_empty());

}




/* Spawn application in the background as a helper function */
fn spawn_app() -> String {
    let listener = TcpListener::bind(RANDOM_PORT)
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = backend_kronos::startup::run_server(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    // return the port to the calling function, so the test goes to the correct port!
    format! ("http://127.0.0.1:{}", port)
}