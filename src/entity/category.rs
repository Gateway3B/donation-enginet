use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub list_id: i32,

    pub name: String,

    #[sea_orm(default_value = "Decimal::ONE")]
    pub multiplier: Decimal,
    pub percent_override: Option<Decimal>,
    pub value_override: Option<Decimal>,

    pub enabled: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::list::Entity",
        from = "Column::ListId",
        to = "super::list::Column::Id"
    )]
    List,
    #[sea_orm(has_many = "super::entry::Entity")]
    Entry,
}

impl Related<super::list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::List.def()
    }
}

impl Related<super::entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
