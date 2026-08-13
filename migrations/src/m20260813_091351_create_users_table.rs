use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_091351_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("picture").null())
                    .col(string("name"))
                    .col(string("email").unique_key())
                    .col(string("phone").unique_key())
                    .col(string("email_verified_at").null())
                    .col(string("password"))
                    .col(timestamp("deleted_at").null())
                    .col(timestamp("created_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(timestamp("updated_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("users").to_owned())
            .await
    }
}
