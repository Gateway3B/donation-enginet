use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "budget")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub list_id: i32,

    #[sea_orm(default_value = "Decimal::new(50_000, 0)")]
    pub total_value: Decimal,
    pub donation_percent: Decimal,
    pub value_override: Option<Decimal>,

    #[sea_orm(ignore)]
    pub donation_value: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::list::Entity",
        from = "Column::ListId",
        to = "super::list::Column::Id"
    )]
    List,
}

impl Related<super::list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::List.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
