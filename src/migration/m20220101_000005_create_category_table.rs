use sea_orm::prelude::Decimal;
use sea_orm_migration::prelude::*;

use super::m20220101_000003_create_list_table::List;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Category {
    Table,
    Id,
    ListId,
    Name,
    ValueMultiplier,
    PercentOverride,
    ValueOverride,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::ListId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Category::Table, Category::ListId)
                            .to(List::Table, List::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .col(
                        ColumnDef::new(Category::ValueMultiplier)
                            .decimal()
                            .default(Decimal::ONE)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Category::PercentOverride).decimal().null())
                    .col(ColumnDef::new(Category::ValueOverride).decimal().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}
