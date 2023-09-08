use sea_orm::prelude::Decimal;
use sea_orm_migration::prelude::*;

use super::m20220101_000005_create_category_table::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Entry {
    Table,
    Id,
    CategoryId,
    Ein,
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
                    .table(Entry::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Entry::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Entry::CategoryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Entry::Table, Entry::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Entry::Ein).integer().not_null())
                    .col(
                        ColumnDef::new(Entry::ValueMultiplier)
                            .decimal()
                            .default(Decimal::ONE)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Entry::PercentOverride).decimal().null())
                    .col(ColumnDef::new(Entry::ValueOverride).decimal().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entry::Table).to_owned())
            .await
    }
}
