use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_223457_create_model_has_permissions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("model_has_permissions")
                    .if_not_exists()
                    // Colonnes composant la clé primaire composite
                    .col(big_integer("permission_id"))
                    .col(string("model_type"))
                    .col(big_integer("model_id"))
                    // Déclaration de la clé primaire composite (permission_id, model_type, model_id)
                    .primary_key(
                        Index::create()
                            .col("permission_id")
                            .col("model_type")
                            .col("model_id"),
                    )
                    // Clé étrangère vers permissions
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_model_has_permissions_permission_id")
                            .from("model_has_permissions", "permission_id")
                            .to("permissions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("model_has_permissions").to_owned())
            .await
    }
}
