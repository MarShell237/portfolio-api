use entities::{likes, posts, projects, users};
use portfolio_api::enums::likeable_enum::LikeableEnum;
use sea_orm::{ActiveEnum, ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260918_183913_like_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let posts = posts::Entity::find().all(db).await?;
        let users = users::Entity::find().all(db).await?;
        let projects = projects::Entity::find().all(db).await?;

        let mut likes_to_insert = Vec::new();
        for post in &posts {
            for i in 0..10 {
                let random_user = &users[(100 * i) % users.len()];
                let active_like = likes::ActiveModel {
                    liker_id: Set(random_user.id),
                    likeable_id: Set(post.id),
                    likeable_type: Set(LikeableEnum::Post.to_value()),
                    ..Default::default()
                };
                likes_to_insert.push(active_like);
            }
        }

        for project in &projects {
            for i in 0..10 {
                let random_user = &users[(100 * i) % users.len()];
                let active_like = likes::ActiveModel {
                    liker_id: Set(random_user.id),
                    likeable_id: Set(project.id),
                    likeable_type: Set(LikeableEnum::Project.to_value()),
                    ..Default::default()
                };
                likes_to_insert.push(active_like);
            }
        }

        let _ = likes::Entity::insert_many(likes_to_insert).exec(db).await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        likes::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }
}
