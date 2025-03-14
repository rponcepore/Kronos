use sea_orm_migration::{prelude::{extension::postgres::*,*}, schema::*};
use backend_kronos::reference::army_echelons::Echelon;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
        .create_type(
            Type::create()
                .as_enum(Echelon::Enum)
                .values([
                    Echelon::Team, 
                    Echelon::Squad,
                    Echelon::Section
                    ])
                .to_owned(),
        )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_type(Type::drop().name(Echelon::Enum).to_owned())
            .await
    }
}

// The standard #[derive] stuff and enum can be found in the module located at line 2.