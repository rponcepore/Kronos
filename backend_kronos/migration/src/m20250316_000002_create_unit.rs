use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Unit {
    Table, // Special case, maped to "Unit" enum. 
    Uic,
    Echelon,
    Nickname,
    DisplayName,
    ShortName,
    Component,
    StateAbbrev,
    ParentUIC,
    Level,
    ServiceMemberCapacity,


}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Unit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Unit::Uic)
                        .string()
                        .not_null()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(Unit::Echelon).string().not_null())
                    .col(ColumnDef::new(Unit::Nickname).string().not_null())
                    .col(ColumnDef::new(Unit::DisplayName).string().not_null())
                    .col(ColumnDef::new(Unit::ShortName).string().not_null())
                    .col(ColumnDef::new(Unit::Component).string().not_null())
                    .col(ColumnDef::new(Unit::StateAbbrev).string().not_null())
                    .col(ColumnDef::new(Unit::Level).integer().not_null())
                    .col(ColumnDef::new(Unit::ServiceMemberCapacity).integer().not_null())
                    .col(ColumnDef::new(Unit::ParentUIC).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-uic-parentuic")
                            .from(Unit::Table, Unit::Uic)
                            .to(Unit::Table, Unit::Uic)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Unit::Table).to_owned())
            .await
    }
}


