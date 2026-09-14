use entities::{comments, likes, projects, shares, tags};
use sea_orm::{
    ActiveEnum, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};

use crate::{
    dtos::project_dtos::{
        ProjectAdjacent, ProjectCollection, ProjectMetrics, ProjectsAdjacentResponse,
    },
    enums::{
        commentable_enum::CommentableEnum, likeable_enum::LikeableEnum,
        shareable_enum::ShareableEnum,
    },
    errors::AppError,
    helpers::pagination_meta::PaginationMeta,
};

pub async fn get_paginate(
    db_pool: &DatabaseConnection,
    page: u64,
) -> Result<(Vec<ProjectCollection>, PaginationMeta), AppError> {
    let paginator = projects::Entity::find()
        .filter(projects::COLUMN.published_at.is_not_null())
        .order_by_desc(projects::COLUMN.id)
        .paginate(db_pool, 12);

    let (projects, meta) = PaginationMeta::paginate(&paginator, page).await?;
    Ok((
        projects
            .into_iter()
            .map(ProjectCollection::from)
            .collect::<Vec<ProjectCollection>>(),
        meta,
    ))
}

pub async fn get_pinned(db_pool: &DatabaseConnection) -> Result<Vec<ProjectCollection>, AppError> {
    let projects = projects::Entity::find()
        .filter(projects::COLUMN.published_at.is_not_null())
        .filter(projects::COLUMN.is_pinned.eq(true))
        .order_by_desc(projects::COLUMN.id)
        .all(db_pool)
        .await?;

    Ok(projects
        .into_iter()
        .map(ProjectCollection::from)
        .collect::<Vec<ProjectCollection>>())
}

pub async fn find_by_slug_or_fail(
    db: &DatabaseConnection,
    slug: &str,
) -> Result<projects::Model, AppError> {
    projects::Entity::find()
        .filter(projects::COLUMN.published_at.is_not_null())
        .filter(projects::COLUMN.slug.eq(slug))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Project not found"))
}

pub async fn find_or_fail(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<projects::Model, AppError> {
    projects::Entity::find()
        .filter(projects::COLUMN.published_at.is_not_null())
        .filter(projects::COLUMN.id.eq(project_id))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Project not found"))
}

pub async fn get_metrics(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<ProjectMetrics, AppError> {
    let comments_count = comments::Entity::find()
        .filter(comments::COLUMN.commentable_id.eq(project_id))
        .filter(
            comments::COLUMN
                .commentable_type
                .eq(CommentableEnum::Project.to_value()),
        )
        .count(db_pool)
        .await?;

    let likes_count = likes::Entity::find()
        .filter(likes::COLUMN.likeable_id.eq(project_id))
        .filter(
            likes::COLUMN
                .likeable_type
                .eq(LikeableEnum::Project.to_value()),
        )
        .count(db_pool)
        .await?;

    let shares_count = shares::Entity::find()
        .filter(shares::COLUMN.sharer_id.eq(project_id))
        .filter(
            shares::COLUMN
                .shareable_type
                .eq(ShareableEnum::Project.to_value()),
        )
        .count(db_pool)
        .await?;

    Ok(ProjectMetrics {
        project_id,
        comments_count,
        likes_count,
        shares_count,
    })
}

pub async fn get_tags(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<Vec<tags::Model>, AppError> {
    let project = find_or_fail(project_id, db_pool).await?;
    project
        .find_related(tags::Entity)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}

pub async fn adjacent(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<ProjectsAdjacentResponse, AppError> {
    let project = find_or_fail(project_id, db_pool).await?;

    let published_at = match project.published_at {
        Some(date) => date,
        None => {
            return Ok(ProjectsAdjacentResponse {
                prev: None,
                next: None,
            });
        }
    };

    let prev_model = projects::Entity::find()
        .filter(projects::COLUMN.published_at.is_not_null())
        .filter(projects::COLUMN.published_at.lt(published_at))
        .order_by_desc(projects::COLUMN.published_at)
        .one(db_pool)
        .await?;

    let next_model = projects::Entity::find()
        .filter(projects::COLUMN.published_at.is_not_null())
        .filter(projects::COLUMN.published_at.gt(published_at))
        .order_by_asc(projects::COLUMN.published_at)
        .one(db_pool)
        .await?;

    let prev = prev_model.map(ProjectAdjacent::from);
    let next = next_model.map(ProjectAdjacent::from);
    Ok(ProjectsAdjacentResponse { prev, next })
}
