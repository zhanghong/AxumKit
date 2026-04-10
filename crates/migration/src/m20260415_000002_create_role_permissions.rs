use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create role_permissions table
        manager
            .create_table(
                Table::create()
                    .table(RolePermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermissions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RolePermissions::Role)
                            .enumeration("role", ["mod", "admin"])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermissions::Permission)
                            .enumeration("permission", [
                                "user:manage",
                                "user:ban",
                                "user:role:manage",
                                "moderation:manage",
                                "action_log:view",
                                "settings:manage",
                                "oauth:manage",
                                "search:manage",
                            ])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermissions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name("idx-role-permission-unique")
                            .col(RolePermissions::Role)
                            .col(RolePermissions::Permission)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop role_permissions table
        manager
            .drop_table(Table::drop().table(RolePermissions::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-orm-migration/#how-to
#[derive(Iden)]
enum RolePermissions {
    Table,
    Id,
    Role,
    Permission,
    CreatedAt,
}
