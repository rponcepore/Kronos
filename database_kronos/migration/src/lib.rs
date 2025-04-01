pub use sea_orm_migration::prelude::*;

// All migrations must be listed here with a mod import
mod m20250220_000001_create_table;
mod m20250316_000002_create_unit;
mod m20250316_000003_create_plan;
mod m20250317_000004_create_kronosorder;
mod m20250317_000005_create_paragraph;
mod m20250319_000006_seed_unit_data;
mod m20250319_000007_seed_plans_data;
mod m20250326_000008_order_template;
mod m20250328_000009_create_template_paragraphs;
mod m20250331_000010_create_order;

// Import the helper modules
mod preloaded_data;
mod helper_methods;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // This vector contains the Box ptrs to all migrations we want to run, matching the "mod"'s above.
        vec![
            Box::new(m20250220_000001_create_table::Migration),
            Box::new(m20250316_000002_create_unit::Migration),
            Box::new(m20250316_000003_create_plan::Migration),
            Box::new(m20250317_000004_create_kronosorder::Migration),
            Box::new(m20250317_000005_create_paragraph::Migration),
            Box::new(m20250319_000006_seed_unit_data::Migration),
            Box::new(m20250319_000007_seed_plans_data::Migration),
            Box::new(m20250326_000008_order_template::Migration),
            Box::new(m20250328_000009_create_template_paragraphs::Migration),
            Box::new(m20250331_000010_create_order::Migration),
        ]
    }
}
