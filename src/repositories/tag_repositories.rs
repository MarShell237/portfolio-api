use crate::errors::AppError;
use entities::{posts, posts_tags, projects, projects_tags, tags};
use migrations::SimpleExpr;
use sea_orm::{
    DatabaseConnection, EntityTrait, JoinType, ModelTrait, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait,
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
        .filter(tags::COLUMN.slug.eq(slug))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Tag not found"))
}

pub async fn find_post_tags_with_count(
    db_pool: &DatabaseConnection,
) -> Result<Vec<tags::Model>, AppError> {
    tags::Entity::find()
        .join(JoinType::InnerJoin, tags::Relation::PostsTags.def())
        .group_by(tags::COLUMN.id)
        .order_by_desc(SimpleExpr::from(posts_tags::COLUMN.post_id.count()))
        .all(db_pool)
        .await
        .map_err(AppError::from)
}

pub async fn find_project_tags_with_count(
    db_pool: &DatabaseConnection,
) -> Result<Vec<tags::Model>, AppError> {
    tags::Entity::find()
        .join(JoinType::InnerJoin, tags::Relation::ProjectsTags.def())
        .group_by(tags::COLUMN.id)
        .order_by_desc(SimpleExpr::from(projects_tags::COLUMN.project_id.count()))
        .all(db_pool)
        .await
        .map_err(AppError::from)
}

pub async fn find_projects_by_tag_slug(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<Vec<projects::Model>, AppError> {
    let tag = find_by_slug_or_fail(db_pool, slug).await?;
    tag.find_related(projects::Entity)
        .filter(projects::COLUMN.published_at.is_not_null())
        .order_by_desc(projects::COLUMN.id)
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
        .filter(posts::COLUMN.published_at.is_not_null())
        .order_by_desc(posts::COLUMN.id)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}
