#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

pub use sea_orm_migration::prelude::*;

#[allow(clippy::wildcard_imports)]
use migrations::*;

mod migrations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20230226_194527_create_user_credentials_table::Migration,
        )]
    }
}
