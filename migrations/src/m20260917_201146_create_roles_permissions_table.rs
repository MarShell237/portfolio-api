use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_201146_create_roles_permissions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("roles_permissions")
                    .if_not_exists()
                    .col(big_integer("permission_id"))
                    .col(big_integer("role_id"))
                    .primary_key(Index::create().col("permission_id").col("role_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_roles_permissions_permission_id")
                            .from("roles_permissions", "permission_id")
                            .to("permissions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_roles_permissions_role_id")
                            .from("roles_permissions", "role_id")
                            .to("roles", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("roles_permissions").to_owned())
            .await
    }
}
