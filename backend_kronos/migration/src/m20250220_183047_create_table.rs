use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(TestTable::Table)
                    .if_not_exists()
                    .col(pk_auto(TestTable::Id))
                    .col(string(TestTable::Title))
                    .col(string(TestTable::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(TestTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TestTable {
    Table, // This is a special case; it will be mapped to the table name, Test_Table
    Id,
    Title,
    Text,
}
