use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_215909_create_post_tag_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("post_tag")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(big_integer("post_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tag_post_id")
                            .from("post_tag", "post_id")
                            .to("posts", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(big_integer("tag_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tag_tag_id")
                            .from("post_tag", "tag_id")
                            .to("tags", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_post_tag_unique")
                            .col("post_id")
                            .col("tag_id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("post_tag").to_owned())
            .await
    }
}
