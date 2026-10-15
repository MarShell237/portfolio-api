use entities::likes::{self, Column::LikeableId};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};

use crate::errors::AppError;

pub async fn already_liked(
    user_id: i64,
    likeable_type: String,
    likeable_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<bool, AppError> {
    Ok(
        find_like_by_user_and_likeable(user_id, likeable_type, likeable_id, db_pool)
            .await?
            .is_some(),
    )
}

async fn find_like_by_user_and_likeable(
    user_id: i64,
    likeable_type: String,
    likeable_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<Option<likes::Model>, AppError> {
    likes::Entity::find()
        .filter(likes::COLUMN.liker_id.eq(user_id))
        .filter(likes::COLUMN.likeable_type.eq(likeable_type))
        .filter(likes::COLUMN.likeable_id.eq(likeable_id))
        .one(db_pool)
        .await
        .map_err(AppError::from)
}
pub async fn unlike(
    user_id: i64,
    likeable_type: String,
    likeable_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<(), AppError> {
    let _ = find_like_by_user_and_likeable(user_id, likeable_type, likeable_id, db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Like not found"))?
        .delete(db_pool)
        .await
        .map_err(AppError::from)?;
    Ok(())
}

pub async fn like(
    user_id: i64,
    likeable_type: String,
    likeable_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<(), AppError> {
    let _ = likes::ActiveModel {
        liker_id: Set(user_id),
        likeable_type: Set(likeable_type),
        likeable_id: Set(likeable_id),
        ..Default::default()
    }
    .insert(db_pool)
    .await
    .map_err(AppError::from);
    Ok(())
}
