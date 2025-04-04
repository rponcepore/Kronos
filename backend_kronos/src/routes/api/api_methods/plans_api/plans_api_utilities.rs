use sea_orm::*;
use crate::models::entities::{prelude, *};

pub async fn get_plan_with_highest_serial(
    db: &DatabaseConnection,
    uic: &String,
    fy: &i32,
) -> Result<Option<plan::Model>, DbErr> {
    plan::Entity::find()
        .filter(plan::Column::Unit.eq(uic))
        .filter(plan::Column::FiscalYear.eq(fy.clone()))
        .order_by_desc(plan::Column::SerialNumber)
        .one(db)
        .await
}