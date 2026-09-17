use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_161115_create_posts_tags_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("posts_tags")
                    .if_not_exists()
                    .col(big_integer("id").auto_increment().primary_key())
                    .col(big_integer("post_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_tags_post_id")
                            .from("posts_tags", "post_id")
                            .to("posts", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(big_integer("tag_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_tags_tag_id")
                            .from("posts_tags", "tag_id")
                            .to("tags", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_posts_tags_unique")
                            .col("post_id")
                            .col("tag_id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("posts_tags").to_owned())
            .await
    }
}
