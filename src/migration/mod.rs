pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_default_category_table;
mod m20220101_000002_create_color_table;
mod m20220101_000003_create_list_table;
mod m20220101_000004_create_budget_table;
mod m20220101_000005_create_category_table;
mod m20220101_000006_create_entry_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_default_category_table::Migration),
            Box::new(m20220101_000002_create_color_table::Migration),
            Box::new(m20220101_000003_create_list_table::Migration),
            Box::new(m20220101_000004_create_budget_table::Migration),
            Box::new(m20220101_000005_create_category_table::Migration),
            Box::new(m20220101_000006_create_entry_table::Migration),
        ]
    }
}
