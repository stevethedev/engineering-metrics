use sea_orm_migration::prelude::*;

use lib_crypto::hash_password;

use crate::sea_orm::prelude::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

const IDX_USERNAME: &str = "idx-user_username";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // CREATE TABLE IF NOT EXISTS "user" (
        //     "id" UUID NOT NULL PRIMARY KEY,
        //     "username" VARCHAR NOT NULL,
        //     "password_hash" VARCHAR NOT NULL,
        //     "is_enabled" BOOLEAN NOT NULL DEFAULT true
        // );
        manager
            .create_table(
                Table::create()
                    .table(UserCredentials::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserCredentials::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserCredentials::Username)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCredentials::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCredentials::IsEnabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .clone(),
            )
            .await?;

        // CREATE INDEX IF NOT EXISTS "idx-user_username" ON "user" ("username");
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(IDX_USERNAME)
                    .table(UserCredentials::Table)
                    .col(UserCredentials::Username)
                    .clone(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(UserCredentials::Table)
            .columns([
                UserCredentials::Id,
                UserCredentials::Username,
                UserCredentials::PasswordHash,
                UserCredentials::IsEnabled,
            ])
            .values_panic([
                Uuid::new_v4().into(),
                "admin".into(),
                hash_password(b"admin").unwrap().into(),
                true.into(),
            ])
            .clone();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // DROP INDEX "idx-user_username";
        manager
            .drop_index(Index::drop().name(IDX_USERNAME).clone())
            .await?;

        // DROP TABLE IF EXISTS "user";
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(UserCredentials::Table)
                    .clone(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at <https://docs.rs/sea-query#iden>
#[derive(Iden)]
enum UserCredentials {
    Table,
    Id,
    Username,
    PasswordHash,
    IsEnabled,
}
