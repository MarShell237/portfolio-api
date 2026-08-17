use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_185036_create_projects_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("projects")
                    .if_not_exists()
                    .col(big_integer("id").auto_increment().primary_key())
                    .col(string("cover_image").null())
                    .col(string("title").unique_key())
                    .col(string("slug").unique_key())
                    .col(text("excerpt"))
                    .col(text("content"))
                    .col(big_integer("estimated_cost"))
                    .col(big_integer("reading_time"))
                    .col(big_integer("views_count").default(0))
                    .col(boolean("is_pinned").default(false))
                    .col(timestamp("published_at").null())
                    .col(timestamp("created_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(timestamp("updated_at").extra("DEFAULT CURRENT_TIMESTAMP"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("projects").to_owned())
            .await
    }
}
