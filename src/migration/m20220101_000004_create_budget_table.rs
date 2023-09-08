use sea_orm::prelude::Decimal;
use sea_orm_migration::prelude::*;

use super::m20220101_000003_create_list_table::List;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Budget {
    Table,
    Id,
    ListId,
    TotalValue,
    DonationPercent,
    ValueOverride,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Budget::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Budget::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Budget::ListId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Budget::Table, Budget::ListId)
                            .to(List::Table, List::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Budget::TotalValue)
                            .decimal()
                            .default(Decimal::new(50_000, 0))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Budget::DonationPercent).decimal().not_null())
                    .col(ColumnDef::new(Budget::ValueOverride).decimal().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Budget::Table).to_owned())
            .await
    }
}
