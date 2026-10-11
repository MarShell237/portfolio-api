use entities::comments;
use sea_orm::{DatabaseConnection, EntityTrait};

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
