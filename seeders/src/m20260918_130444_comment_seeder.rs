use entities::{comments, posts, projects, users};
use portfolio_api::enums::commentable_enum::CommentableEnum;
use sea_orm::{ActiveEnum, ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260918_130444_comment_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let posts = posts::Entity::find().all(db).await?;
        let users = users::Entity::find().all(db).await?;
        let projects = projects::Entity::find().all(db).await?;

        let mut comments_to_insert = Vec::new();
        for post in &posts {
            for i in 0..10 {
                let random_user = &users[(100 * i) % users.len()];

                let mut active_comment = factories::comment_factory::create();

                active_comment.commenter_id = Set(random_user.id);
                active_comment.commentable_id = Set(post.id);
                active_comment.commentable_type = Set(CommentableEnum::Post.to_value());
                comments_to_insert.push(active_comment);
            }
        }

        for project in &projects {
            for i in 0..10 {
                let random_user = &users[(100 * i) % users.len()];

                let mut active_comment = factories::comment_factory::create();

                active_comment.commenter_id = Set(random_user.id);
                active_comment.commentable_id = Set(project.id);
                active_comment.commentable_type = Set(CommentableEnum::Project.to_value());
                comments_to_insert.push(active_comment);
            }
        }

        let _ = comments::Entity::insert_many(comments_to_insert)
            .exec(db)
            .await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        comments::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }
}
