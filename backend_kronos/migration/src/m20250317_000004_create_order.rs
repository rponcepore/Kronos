use sea_orm_migration::{prelude::*, schema::*};

use super::m20250316_000003_create_plan::Plan;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Order {
    Table,
    Id,
    ParentPlan,
    OrderType,
    SerialNumber,
    IsPublished,
    DerivedFrom,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(Order::ParentPlan).integer().not_null())
                    .foreign_key( // order foreign key pointing to plan pk for which plan it belongs to
                        ForeignKey::create() //bind the plan to the authoring unit
                            .name("fk-order-id-plan-planid")
                            .from(Order::Table, Order::Id)
                            .to(Plan::Table, Plan::Id)
                            .on_delete(ForeignKeyAction::Cascade) // delete this item if the plan is deleted
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Order::OrderType))
                    .col(integer(Order::SerialNumber))
                    .col(boolean(Order::IsPublished))
                    .col(
                        ColumnDef::new(Order::DerivedFrom)
                        .integer()
                    )
                    .foreign_key( // order self join to derive orders from other orders. Possibly unnecessary.
                        ForeignKey::create() //bind the plan to the authoring unit
                            .name("fk-order-id-order-derived-from")
                            .from(Order::Table, Order::DerivedFrom)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await
    }
}


