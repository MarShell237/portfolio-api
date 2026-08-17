use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_223424_create_permissions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("permissions")
                    .if_not_exists()
                    .col(big_integer("id").auto_increment().primary_key())
                    .col(string("name"))
                    .col(string("guard_name"))
                    .col(timestamp("created_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(timestamp("updated_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    // Contrainte unique sur (name, guard_name)
                    .index(
                        Index::create()
                            .unique()
                            .name("permissions_name_guard_name_unique")
                            .col("name")
                            .col("guard_name"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("permissions").to_owned())
            .await
    }
}
