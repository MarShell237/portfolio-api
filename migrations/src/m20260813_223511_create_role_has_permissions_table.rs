use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_223511_create_role_has_permissions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("role_has_permissions")
                    .if_not_exists()
                    // Colonnes composant la clé primaire composite
                    .col(big_integer("permission_id"))
                    .col(big_integer("role_id"))
                    // Clé primaire composite
                    .primary_key(Index::create().col("permission_id").col("role_id"))
                    // Clé étrangère vers permissions
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_has_permissions_permission_id")
                            .from("role_has_permissions", "permission_id")
                            .to("permissions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Clé étrangère vers roles
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_has_permissions_role_id")
                            .from("role_has_permissions", "role_id")
                            .to("roles", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("role_has_permissions").to_owned())
            .await
    }
}
