use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "entry")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub category_id: i32,

    pub ein: i32,

    #[sea_orm(default_value = "Decimal::ONE")]
    pub multiplier: Decimal,
    pub percent_override: Option<Decimal>,
    pub value_override: Option<Decimal>,

    pub enabled: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
