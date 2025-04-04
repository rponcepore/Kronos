use sea_orm_migration::prelude::*;

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

        let mut plan_vec: Vec<(&str, i32, i32, &str, &str)> = Vec::new();
        plan_vec.push(("WJH8AA", 25, 1, "Blackbeard", "CUI"));
        plan_vec.push(("WJH8AA", 25, 2, "Revenge", "CUI"));
        plan_vec.push(("WJH8AA", 25, 3, "Jack Sparrow", "CUI"));
        plan_vec.push(("WJH8AA", 25, 3, "Pirate's Life", "CUI"));

        for plan in plan_vec{
            let insert = Query::insert()
                .into_table(Plan::Table)
                .columns([  Plan::Uic, 
                            Plan::FiscalYear, 
                            Plan::SerialNumber, 
                            Plan::Name, 
                            Plan::Classification])
                .values_panic([
                            plan.0.into(),
                            plan.1.into(),
                            plan.2.into(),
                            plan.3.into(),
                            plan.4.into(),
                        ]) 
                .to_owned();

            manager.exec_stmt(insert).await?;
        }
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt(
            Query::delete()
                .from_table(Plan::Table)
                .cond_where(Expr::col(Plan::Name).is_in(["Blackbeard", "Revenge", "Jack Sparrow", "Pirate's Life"]))
                .to_owned()
        ).await?;

        Ok(())
    }
}

