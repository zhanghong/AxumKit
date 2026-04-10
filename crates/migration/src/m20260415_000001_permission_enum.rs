use sea_orm_migration::prelude::*;
use sea_orm_migration::prelude::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create permission enum type
        manager
            .create_type(
                Type::create()
                    .as_enum("permission")
                    .values([
                        "user:manage",
                        "user:ban",
                        "user:role:manage",
                        "moderation:manage",
                        "action_log:view",
                        "settings:manage",
                        "oauth:manage",
                        "search:manage",
                    ])
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop permission enum type
        manager
            .drop_type(Type::drop().name("permission").to_owned())
            .await
    }
}
