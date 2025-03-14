pub use sea_orm_migration::prelude::*;

// All migrations must be listed here with a mod import
mod m20250220_000001_create_table;
mod m20250314_000002_create_echelon_type;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // This vector contains the Box ptrs to all migrations we want to run, matching the "mod"'s above.
        vec![
            Box::new(m20250220_000001_create_table::Migration),
            Box::new(m20250314_000002_create_echelon_type::Migration),
        ]
    }
}
