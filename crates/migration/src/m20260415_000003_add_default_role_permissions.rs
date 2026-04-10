use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        
        // Insert default permissions for admin role
        let admin_permissions = vec![
            "user:manage",
            "user:ban",
            "user:role:manage",
            "moderation:manage",
            "action_log:view",
            "settings:manage",
            "oauth:manage",
            "search:manage",
        ];

        for permission in admin_permissions {
            conn.execute_unprepared(
                &format!(
                    "INSERT INTO role_permissions (role, permission, created_at) 
                     VALUES ('admin', '{}', NOW()) 
                     ON CONFLICT (role, permission) DO NOTHING",
                    permission
                )
            )
            .await?;
        }

        // Insert default permissions for mod role
        let mod_permissions = vec![
            "user:ban",
            "moderation:manage",
            "action_log:view",
        ];

        for permission in mod_permissions {
            conn.execute_unprepared(
                &format!(
                    "INSERT INTO role_permissions (role, permission, created_at) 
                     VALUES ('mod', '{}', NOW()) 
                     ON CONFLICT (role, permission) DO NOTHING",
                    permission
                )
            )
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        // Remove all role permissions
        conn.execute_unprepared("DELETE FROM role_permissions")
            .await?;

        Ok(())
    }
}
