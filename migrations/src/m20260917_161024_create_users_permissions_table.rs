use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_161024_create_users_permissions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("users_permissions")
                    .if_not_exists()
                    .col(big_integer("user_id"))
                    .col(big_integer("permission_id"))
                    .primary_key(Index::create().col("user_id").col("permission_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_permissions_user_id")
                            .from("users_permissions", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_permissions_permission_id")
                            .from("users_permissions", "permission_id")
                            .to("permissions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("users_permissions").to_owned())
            .await
    }
}
