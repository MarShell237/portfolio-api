use crate::errors::AppError;
use entities::tags::{Column as TagColumn, Entity as Tag, Model as TagModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn find_by_slug_or_fail(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<TagModel, AppError> {
    Tag::find()
        .filter(TagColumn::Slug.eq(slug))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Tag not found"))
}
