use entities::{posts, projects, shares, users};
use portfolio_api::enums::shareable_enum::ShareableEnum;
use sea_orm::{ActiveEnum, ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260918_185439_share_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let posts = posts::Entity::find().all(db).await?;
        let users = users::Entity::find().all(db).await?;
        let projects = projects::Entity::find().all(db).await?;

        let mut shares_to_insert = Vec::new();
        for post in &posts {
            for i in 0..10 {
                let random_user = &users[(100 * i) % users.len()];
                let active_share = shares::ActiveModel {
                    sharer_id: Set(random_user.id),
                    shareable_id: Set(post.id),
                    shareable_type: Set(ShareableEnum::Post.to_value()),
                    ..Default::default()
                };
                shares_to_insert.push(active_share);
            }
        }

        for project in &projects {
            for i in 0..10 {
                let random_user = &users[(100 * i) % users.len()];
                let active_share = shares::ActiveModel {
                    sharer_id: Set(random_user.id),
                    shareable_id: Set(project.id),
                    shareable_type: Set(ShareableEnum::Project.to_value()),
                    ..Default::default()
                };
                shares_to_insert.push(active_share);
            }
        }

        let _ = shares::Entity::insert_many(shares_to_insert).exec(db).await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        shares::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }
}
