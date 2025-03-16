use sea_orm_migration::{prelude::*, schema::*};

use super::m20250316_000002_create_unit::Unit;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Plan {
    Table, // Special case, maped to "Unit" enum. 
    PlanId,
    Unit,
    ParentPlan,
    FiscalYear,
    SerialNumber,
    Name,
    Classification,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Plan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Plan::PlanId)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(Plan::Unit).string().not_null())
                    .col(ColumnDef::new(Plan::ParentPlan).integer())
                    .col(ColumnDef::new(Plan::FiscalYear).integer().not_null())
                    .col(ColumnDef::new(Plan::SerialNumber).integer().not_null())
                    .col(ColumnDef::new(Plan::Classification).string().not_null())
                    .col(ColumnDef::new(Plan::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create() //bind the plan to the authoring unit
                            .name("fk-plan-unit-unit-uic")
                            .from(Plan::Table, Plan::Unit)
                            .to(Unit::Table, Unit::Uic)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create() //optionally bind the plan to the parent plan.
                            .name("fk-plan-planid-plan-planid")
                            .from(Plan::Table, Plan::PlanId)
                            .to(Plan::Table, Plan::PlanId)
                            .on_update(ForeignKeyAction::Cascade), //TODO: is this a good idea?
                    )
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Plan::Table).to_owned())
            .await
    }
}


