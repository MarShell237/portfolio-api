use entities::shares;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, QueryFilter};

use crate::errors::AppError;

pub async fn already_shared(
    shareable_type: String,
    shareable_id: i64,
    user_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<bool, AppError> {
    Ok(shares::Entity::find()
        .filter(shares::COLUMN.shareable_type.eq(shareable_type))
        .filter(shares::COLUMN.shareable_id.eq(shareable_id))
        .filter(shares::COLUMN.sharer_id.eq(user_id))
        .one(db_pool)
        .await?
        .is_some())
}

pub async fn store(
    shareable_type: String,
    shareable_id: i64,
    user_id: i64,
    platform: String,
    db_pool: &DatabaseConnection,
) -> Result<(), AppError> {
    shares::ActiveModel {
        sharer_id: Set(user_id),
        shareable_id: Set(shareable_id),
        shareable_type: Set(shareable_type),
        platform: Set(platform),
        ..Default::default()
    }
    .insert(db_pool)
    .await
    .map(|_| ())
    .map_err(AppError::from)
}
