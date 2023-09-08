use futures::future::join_all;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::prelude::*;

use crate::entity::color;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Color {
    Table,
    Id,
    Name,
    Value,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Color::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Color::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Color::Name).string().not_null())
                    .col(ColumnDef::new(Color::Value).string().not_null())
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        let colors = vec![
            ("Red", "#FF0000"),
            ("Orange", "#FF8000"),
            ("Brown", "#604525"),
            ("Yellow", "#FFFF00"),
            ("Lime Green", "#00FF00"),
            ("Cyan", "#00FF80"),
            ("Aqua", "#00FFFF"),
            ("Light Blue", "#0080FF"),
            ("Blue", "#0000FF"),
            ("Pink", "#FF007F"),
        ];

        let colors_setup = colors.iter().map(|(name, value)| {
            color::ActiveModel {
                name: Set(name.to_string()),
                value: Set(value.to_string()),
                ..Default::default()
            }
            .insert(db)
        });

        let result = join_all(colors_setup)
            .await
            .into_iter()
            .collect::<Result<Vec<color::Model>, DbErr>>();

        result.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Color::Table).to_owned())
            .await
    }
}
