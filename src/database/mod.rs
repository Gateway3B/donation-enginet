use crate::migration::Migrator;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use sea_orm_migration::MigratorTrait;
use std::{env, time::Duration};

pub async fn init_database() -> DatabaseConnection {
    dotenvy::dotenv().expect("Unable to read .env file.");

    let postgres_user = env::var("POSTGRES_USER").expect("Unable to load POSTGRES_USER from env.");
    let postgres_password =
        env::var("POSTGRES_PASSWORD").expect("Unable to load POSTGRES_PASSWORD from env.");
    let postgres_db = env::var("POSTGRES_DB").expect("Unable to load POSTGRES_DB from env.");
    let postgres_schema_name =
        env::var("POSTGRES_SCHEMA_NAME").expect("Unable to load POSTGRES_SCHEMA_NAME from env.");
    let postgres_host = env::var("POSTGRES_HOST").expect("Unable to load POSTGRES_HOST from env.");

    let mut opt = ConnectOptions::new(format!(
        "postgres://{postgres_user}:{postgres_password}@{postgres_host}/{postgres_db}"
    ));
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path(postgres_schema_name.clone());

    let db = Database::connect(opt.clone())
        .await
        .expect("Unable to make connection to db.");

    // Postgres does not allow prepared statements for these statements.
    let create_schema = Statement::from_string(
        DbBackend::Postgres,
        format!(r#"CREATE SCHEMA IF NOT EXISTS "{postgres_schema_name}""#),
    );
    db.execute(create_schema)
        .await
        .expect("Unable to create schema.");

    let grant_schema_permissions = Statement::from_string(
        DbBackend::Postgres,
        format!(
            r#"GRANT ALL ON ALL TABLES IN SCHEMA "{postgres_schema_name}" TO "{postgres_user}""#
        ),
    );
    db.execute(grant_schema_permissions)
        .await
        .expect("Unable to grant schema permissions.");

    let set_search_path = Statement::from_string(
        DbBackend::Postgres,
        format!(r#"ALTER ROLE "{postgres_user}" SET search_path TO "{postgres_schema_name}""#),
    );
    db.execute(set_search_path)
        .await
        .expect("Unable to set search path.");

    Migrator::up(&db, None)
        .await
        .expect("Unable to run migrations.");

    db
}
