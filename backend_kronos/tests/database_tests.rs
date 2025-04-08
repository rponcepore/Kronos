//! tests/local_tests.rs

use sea_orm::*;

use backend_kronos::configuration::{self, get_configuration};
use backend_kronos::models::entities::{prelude::*, *};

#[tokio::test]
async fn database_alive_test() {
    // let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();

    // Now attempt to connect to the database.
    match Database::connect(connection_string).await {
        Ok(..) => {}
        Err(e) => panic!("Failed to connect to database: {}", e),
    };
}

#[tokio::test]
async fn database_crud_test() {
    // let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();

    // Now attempt to connect to the database.
    let connection = Database::connect(connection_string)
        .await
        .expect("Failed to connect to database.");

    // First, clean the table:
    let test_records: Vec<test_table::Model> = match TestTable::find().all(&connection).await {
        Ok(test_records) => test_records,
        Err(e) => panic!("{}", e),
    };

    // Delete each record from the vector
    for record in test_records {
        let _ = match record.delete(&connection).await {
            Ok(_) => {}
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
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    };

    // Find by the id
    let test_record_single: Option<test_table::Model> =
        match TestTable::find_by_id(0).one(&connection).await {
            Ok(test_record_single) => test_record_single,
            Err(e) => panic!("{}", e),
        };

    assert_eq!(test_record_single.unwrap().id, 0);

    // Find the record by filter matching
    let test_record_filter_one: Option<test_table::Model> = match TestTable::find()
        .filter(test_table::Column::Text.eq("Test_Text"))
        .one(&connection)
        .await
    {
        Ok(test_record_filter_one) => test_record_filter_one,
        Err(e) => panic!("{}", e),
    };

    assert_eq!(test_record_filter_one.unwrap().id, 0);

    // Finally, delete the entry (and any duplicate others, too)
    let test_records: Vec<test_table::Model> = match TestTable::find().all(&connection).await {
        Ok(test_records) => test_records,
        Err(e) => panic!("{}", e),
    };

    // Delete each record from the vector
    for record in test_records {
        let _ = match record.delete(&connection).await {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };
    }

    // Ensure that they were deleted
    let test_records: Vec<test_table::Model> = match TestTable::find().all(&connection).await {
        Ok(test_records) => test_records,
        Err(e) => panic!("{}", e),
    };

    assert!(test_records.is_empty());
}
