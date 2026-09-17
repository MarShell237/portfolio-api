use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_161123_create_projects_tags_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("projects_tags")
                    .if_not_exists()
                    .col(big_integer("id").auto_increment().primary_key())
                    .col(big_integer("project_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_projects_tags_project_id")
                            .from("projects_tags", "project_id")
                            .to("projects", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(big_integer("tag_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_projects_tags_tag_id")
                            .from("projects_tags", "tag_id")
                            .to("tags", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_projects_tags_unique")
                            .col("project_id")
                            .col("tag_id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("projects_tags").to_owned())
            .await
    }
}
