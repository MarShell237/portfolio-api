use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_160946_create_users_roles_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("users_roles")
                    .if_not_exists()
                    .col(big_integer("user_id"))
                    .col(big_integer("role_id"))
                    .primary_key(Index::create().col("user_id").col("role_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_roles_user_id")
                            .from("users_roles", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_roles_role_id")
                            .from("users_roles", "role_id")
                            .to("roles", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("users_roles").to_owned())
            .await
    }
}
