//! database.rs

/*
 * This file contains the logic of the ORM used to connect to the database.
 *  */

 /*

 
use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

const DATABASE_URL: &str = "mysql://root:root@localhost:3306";
const DB_NAME: &str = "kronos_db";

pub async fn run_database() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_db_connection() {
        if let Err(err) = block_on(run_database()) {
            panic!("{}", err);
        }
    }
}

     */