use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261017_091444_create_messages_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("messages")
                    .if_not_exists()
                    .col(big_integer("id").auto_increment().primary_key())
                    .col(string("name"))
                    .col(string("phone"))
                    .col(string("email"))
                    .col(string("subject"))
                    .col(string("message"))
                    .col(timestamp("read_at").null())
                    .col(timestamp("created_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(timestamp("updated_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("messages").to_owned())
            .await
    }
}
