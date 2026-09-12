use entities::{post_tag, posts};
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260904_161323_post_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let posts_data: Vec<posts::ActiveModel> =
            (0..8).map(|_| factories::post_factory::create()).collect();

        let inserted_posts = posts::Entity::insert_many(posts_data)
            .exec_with_returning(db)
            .await?;

        let mut pivot_relations = Vec::new();

        for post in inserted_posts {
            let tag_base_id = post.id;

            for offset in 0..3 {
                pivot_relations.push(post_tag::ActiveModel {
                    post_id: Set(post.id),
                    tag_id: Set(tag_base_id + offset),
                    ..Default::default()
                });
            }
        }

        post_tag::Entity::insert_many(pivot_relations)
            .exec(db)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        post_tag::Entity::delete_many().exec(db).await?;
        posts::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
