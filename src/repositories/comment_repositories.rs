use entities::comments;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::errors::AppError;

pub async fn find_or_fail(
    comment_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<comments::Model, AppError> {
    comments::Entity::find_by_id(comment_id)
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Comment not found"))
}

pub async fn create(
    user_id: i64,
    commentable_type: String,
    commentable_id: i64,
    content: String,
    db_pool: &DatabaseConnection,
) -> Result<comments::Model, AppError> {
    comments::ActiveModel {
        commenter_id: Set(user_id),
        commentable_type: Set(commentable_type),
        commentable_id: Set(commentable_id),
        content: Set(content),
        ..Default::default()
    }
    .insert(db_pool)
    .await
    .map_err(AppError::from)
}
