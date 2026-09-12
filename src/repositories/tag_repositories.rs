use crate::errors::AppError;
use entities::{post_tag, posts, project_tag, projects, tags};
use migrations::SimpleExpr;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect,
};

// a effacer
pub async fn all(db_pool: &DatabaseConnection) -> Result<Vec<tags::Model>, AppError> {
    Ok(tags::Entity::find().all(db_pool).await?)
}

pub async fn find_by_slug_or_fail(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<tags::Model, AppError> {
    tags::Entity::find()
        .filter(tags::Column::Slug.eq(slug))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Tag not found"))
}

pub async fn find_post_tags_with_count(
    db_pool: &DatabaseConnection,
) -> Result<Vec<tags::Model>, AppError> {
    // post_tag::Entity::find()
    //     .find_related(tags::Entity)
    //     .group_by(tags::Column::Id)
    //     .order_by_desc(SimpleExpr::from(post_tag::Column::PostId.count()))
    //     .all(db_pool)
    //     .await
    //     .map_err(AppError::from)
    Ok(tags::Entity::find().all(db_pool).await?)
}

pub async fn find_project_tags_with_count(
    db_pool: &DatabaseConnection,
) -> Result<Vec<tags::Model>, AppError> {
    // project_tag::Entity::find()
    //     .find_related(tags::Entity)
    //     .group_by(tags::Column::Id)
    //     .order_by_desc(SimpleExpr::from(project_tag::Column::ProjectId.count()))
    //     .all(db_pool)
    //     .await
    //     .map_err(AppError::from)
    Ok(tags::Entity::find().all(db_pool).await?)
}
pub async fn find_projects_by_tag_slug(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<Vec<projects::Model>, AppError> {
    let tag = find_by_slug_or_fail(db_pool, slug).await?;
    tag.find_related(projects::Entity)
        .filter(projects::Column::PublishedAt.is_not_null())
        .order_by_desc(projects::Column::Id)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}

pub async fn find_posts_by_tag_slug(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<Vec<posts::Model>, AppError> {
    let tag = find_by_slug_or_fail(db_pool, slug).await?;
    tag.find_related(posts::Entity)
        .filter(posts::Column::PublishedAt.is_not_null())
        .order_by_desc(posts::Column::Id)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}
