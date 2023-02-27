use sea_orm_migration::prelude::*;

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
                    .to_owned(),
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
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // DROP INDEX "idx-user_username";
        manager
            .drop_index(Index::drop().name(IDX_USERNAME).to_owned())
            .await?;

        // DROP TABLE IF EXISTS "user";
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(UserCredentials::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserCredentials {
    Table,
    Id,
    Username,
    PasswordHash,
    IsEnabled,
}
