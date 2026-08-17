use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_192818_create_comments_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("comments")
                    .if_not_exists()
                    .col(big_integer("id").auto_increment().primary_key())
                    .col(big_integer("commenter_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_commenter_id")
                            .from("comments", "commenter_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(big_integer("parent_id").null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_parent_id")
                            .from("comments", "parent_id")
                            .to("comments", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(string("commentable_type"))
                    .col(big_integer("commentable_id"))
                    .col(string("attachment").null())
                    .col(text("content"))
                    .col(timestamp("created_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(timestamp("updated_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("comments").to_owned())
            .await
    }
}
