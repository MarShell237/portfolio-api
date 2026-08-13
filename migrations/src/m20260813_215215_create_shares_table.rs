use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_215215_create_shares_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("shares")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(big_integer("sharer_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_shares_sharer_id")
                            .from("shares", "sharer_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(string("shareable_type"))
                    .col(big_integer("shareable_id"))
                    .col(string("platform"))
                    .col(timestamp("shared_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("shares").to_owned())
            .await
    }
}
