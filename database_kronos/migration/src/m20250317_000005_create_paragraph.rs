use sea_orm_migration::{prelude::*, schema::*};

use super::m20250317_000004_create_order::KronosOrder;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Paragraph {
    Table,
    Id,
    KronosOrder,
    ParentParagraph,
    IsMajor,
    OrdinalSequence,
    Title,
    Text,
    IndentLevel, // 0 is base for paragraphs 1-5.
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Paragraph::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Paragraph::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Paragraph::KronosOrder)
                        .integer()
                        .not_null()
                    )
                    .foreign_key( 
                        ForeignKey::create() // bind the paragraph to the owning order
                            .name("fk-paragraph-order-order-id")
                            .from(Paragraph::Table, Paragraph::KronosOrder)
                            .to(KronosOrder::Table, KronosOrder::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Paragraph::ParentParagraph))
                    .foreign_key(
                        ForeignKey::create() //bind the paragraph to its owning paragraph
                            .name("fk-paragraph-parentParagraph-paragraph-id")
                            .from(Paragraph::Table, Paragraph::ParentParagraph)
                            .to(Paragraph::Table, Paragraph::Id)
                            .on_delete(ForeignKeyAction::Cascade) // Save us some business logic, nice.
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(boolean(Paragraph::IsMajor).not_null())
                    .col(integer(Paragraph::OrdinalSequence).not_null())
                    .col(string(Paragraph::Title).not_null())
                    .col(string(Paragraph::Text).not_null())
                    .col(integer(Paragraph::IndentLevel).not_null())
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Paragraph::Table).to_owned())
            .await
    }
}

