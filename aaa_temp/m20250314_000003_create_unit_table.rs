use sea_orm_migration::{prelude::*, schema::*};
// use backend_kronos::reference::army_echelons::Echelon;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Unit::Table)
                    .if_not_exists()
                    .col(
                        // Define 'id' as an auto-incrementing primary key
                        ColumnDef::new(TestTable::Uic)
                            .integer()
                            .not_null()
                            .primary_key(), 
                    )
                    .col(string(Unit::Display_Name))
                    .col(string(Unit::Short_Name))
                    .col(string(Unit::Echelon))
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
enum Unit {
    Table, // This is a special case; it will be mapped to the table name, Test_Table
    Uic,
    Parent_UIC,
    Echelon,
    Short_Name,
    Display_Name,
    Nick_Name,
    Component,
    State,
    Level,
    Capacity,
    

}
