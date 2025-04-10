use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

// Bring plans table into scope
use super::m20250316_000002_create_unit::Unit;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // Step 0: Create a unit for this plan to live in. We'll call that unit "TEMPLT" and it will house orders templates.
        // Hacky but deal with it.
        let template_unit = ("TEMPLT", "UNK", "TEMPLATES", "ORDERS TEMPLATS", "TEMPLATES", "Active");
        let insert_template = Query::insert()
            .into_table(Unit::Table)
            .columns([  Unit::Uic, 
                Unit::Echelon, 
                Unit::Nickname, 
                Unit::DisplayName, 
                Unit::ShortName,
                Unit::Component])
            .values_panic([
                template_unit.0.into(),
                template_unit.1.into(),
                template_unit.2.into(),
                template_unit.3.into(),
                template_unit.4.into(),
                template_unit.5.into(),
            ]) 
            .to_owned();

        manager.exec_stmt(insert_template).await?;

        // First insert the squadron
        let rattlesnake_sqdn = ("WJH8AA", "BN", "Rattlesnake", "2-14 Cavalry Regiment", "2-14 CAV", "Active");
        let insert_sqdn = Query::insert()
            .into_table(Unit::Table)
            .columns([  Unit::Uic, 
                Unit::Echelon, 
                Unit::Nickname, 
                Unit::DisplayName, 
                Unit::ShortName,
                Unit::Component])
            .values_panic([
                rattlesnake_sqdn.0.into(),
                rattlesnake_sqdn.1.into(),
                rattlesnake_sqdn.2.into(),
                rattlesnake_sqdn.3.into(),
                rattlesnake_sqdn.4.into(),
                rattlesnake_sqdn.5.into(),
            ]) 
            .to_owned();

        manager.exec_stmt(insert_sqdn).await?;

        let mut unit_vec: Vec<(&str, &str, &str, &str, &str, &str)> = Vec::new();
        unit_vec.push(("WJH8C0", "CO", "Charlie", "Charlie Troop", "C TRP", "Active"));
        unit_vec.push(("WJH8A0", "CO", "Ace High", "Ace Troop", "A TRP", "Active"));
        unit_vec.push(("WJH8B0", "CO", "Bounty Hunter", "Bounty Troop", "B TRP", "Active"));
        unit_vec.push(("WJH8T0", "CO", "Diablo", "Diablo Troop", "D TRP", "Active"));
        unit_vec.push(("WJH8H0", "CO", "HeadHunter", "Headhunter Troop", "HHT", "Active"));
        
        for unit in unit_vec{
            let insert = Query::insert()
                .into_table(Unit::Table)
                .columns([  Unit::Uic, 
                            Unit::Echelon, 
                            Unit::Nickname, 
                            Unit::DisplayName, 
                            Unit::ShortName,
                            Unit::Component,
                            Unit::ParentUIC])
                .values_panic([
                            unit.0.into(),
                            unit.1.into(),
                            unit.2.into(),
                            unit.3.into(),
                            unit.4.into(),
                            unit.5.into(),
                            rattlesnake_sqdn.0.into(), //they all belong to the Rattlesnake Squadron.
                        ]) 
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt(
            Query::delete()
                .from_table(Unit::Table)
                .cond_where(Expr::col(Unit::Uic).is_in(["WJH8AA", "WJH8T0", "WJH8B0", "WJH8A0", "WJH8C0"]))
                .to_owned()
        ).await?;

        Ok(())
    }
}

