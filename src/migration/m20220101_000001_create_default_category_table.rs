use futures::future::join_all;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::prelude::*;

use crate::entity::default_category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum DefaultCategory {
    Table,
    Id,
    Name,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DefaultCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DefaultCategory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DefaultCategory::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        let default_categories = vec![
            "Animal Welfare",
            "Culture",
            "Education",
            "Environment",
            "Global Health",
            "Health Care",
            "Humanitarian Aid",
            "Justice",
            "Local Causes",
            "Scientific Research",
        ];

        let default_categories_setup = default_categories.iter().map(|name| {
            default_category::ActiveModel {
                name: Set(name.to_string()),
                ..Default::default()
            }
            .insert(db)
        });

        let result = join_all(default_categories_setup)
            .await
            .into_iter()
            .collect::<Result<Vec<default_category::Model>, DbErr>>();

        result.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DefaultCategory::Table).to_owned())
            .await
    }
}
