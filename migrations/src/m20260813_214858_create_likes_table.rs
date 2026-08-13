use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_214858_create_likes_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("likes")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(big_integer("liker_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_likes_liker_id")
                            .from("likes", "liker_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(string("likeable_type"))
                    .col(big_integer("likeable_id"))
                    .col(timestamp("liked_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("likes").to_owned())
            .await
    }
}
