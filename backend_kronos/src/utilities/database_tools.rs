//! database_tools.rs

pub async fn access_kronos_database() -> Result<DatabaseConnection, DbErr> {

    /*
     * This needs to be refactored. We should attempt to store the connection pool
     * (DatabaseConnection) in the application state. However, it needs to be initialized
     * somewhere. We also need to access it in a way that first checks if web::Data:: 
     * has a connection, and if it does, use that. Else, attempt to reconnect. If it fails,
     * handle accordingly. If the connection succeeds, replace the old application state with
     * the new application state. This way, we don't crash the whole thing if the database crashes.
     */

    let configuration = get_configuration().expect("Failed to read configuration.");
    dprintln!("Configuration read successfully.");
    let connection_string = configuration.database.connection_string();
    dprintln!("Connection string: {}", connection_string);
    Database::connect(connection_string).await
}