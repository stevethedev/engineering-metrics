pub use sea_orm_migration::prelude::*;

mod m20230226_194527_create_user_credentials_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230226_194527_create_user_credentials_table::Migration),
            Box::new(m20230227_003003_entity::Migration),
        ]
    }
}
