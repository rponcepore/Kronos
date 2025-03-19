use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

// Bring plans table into scope
use super::m20250316_000003_create_plan::Plan;
/*
pub enum Plan {
    Table, // Special case, maped to "Unit" enum. 
    Id,
    Unit,
    ParentPlan,
    FiscalYear,
    SerialNumber,
    Name,
    Classification,
}
*/
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt(
            Query::insert()
                .into_table(Plan::Table)
                .columns([Plan::Name])
                .values_panic(["Tiramisu".into()])
                .values_panic(["Cheesecake".into()])
                .values_panic(["Chocolate Cake".into()])
                .to_owned()
        ).await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt(
            Query::delete()
                .from_table(Cake::Table)
                .cond_where(Expr::col(Cake::Name).is_in(["Tiramisu", "Cheesecake", "Chocolate Cake"]))
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
