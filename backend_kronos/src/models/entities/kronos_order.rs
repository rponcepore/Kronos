//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "kronos_order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub parent_plan: i32,
    pub order_type: String,
    pub serial_number: i32,
    pub is_published: bool,
    pub derived_from: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::DerivedFrom",
        to = "Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::paragraph::Entity")]
    Paragraph,
    #[sea_orm(
        belongs_to = "super::plan::Entity",
        from = "Column::ParentPlan",
        to = "super::plan::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Plan,
}

impl Related<super::paragraph::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Paragraph.def()
    }
}

impl Related<super::plan::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plan.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
