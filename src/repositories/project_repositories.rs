use entities::{comments, likes, projects, shares};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
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
        .filter(projects::Column::PublishedAt.is_not_null())
        .order_by_desc(projects::Column::Id)
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
        .filter(projects::Column::PublishedAt.is_not_null())
        .filter(projects::Column::IsPinned.eq(true))
        .order_by_desc(projects::Column::Id)
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
        .filter(projects::Column::PublishedAt.is_not_null())
        .filter(projects::Column::Slug.eq(slug))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Project not found"))
}

pub async fn find_or_fail(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<projects::Model, AppError> {
    projects::Entity::find()
        .filter(projects::Column::PublishedAt.is_not_null())
        .filter(projects::Column::Id.eq(project_id))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Project not found"))
}

pub async fn get_metrics(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<ProjectMetrics, AppError> {
    let comments_count = comments::Entity::find()
        .filter(comments::Column::CommentableId.eq(project_id))
        .filter(comments::Column::CommentableType.eq(CommentableEnum::Project))
        .count(db_pool)
        .await?;

    let likes_count = likes::Entity::find()
        .filter(likes::Column::LikeableId.eq(project_id))
        .filter(likes::Column::LikeableType.eq(LikeableEnum::Project))
        .count(db_pool)
        .await?;

    let shares_count = shares::Entity::find()
        .filter(shares::Column::SharerId.eq(project_id))
        .filter(shares::Column::ShareableType.eq(ShareableEnum::Project))
        .count(db_pool)
        .await?;

    Ok(ProjectMetrics {
        project_id,
        comments_count,
        likes_count,
        shares_count,
    })
}

pub async fn adjacent(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<ProjectsAdjacentResponse, AppError> {
    let project = find_or_fail(project_id, db_pool).await?;

    let prev_model = projects::Entity::find()
        .filter(projects::Column::PublishedAt.is_not_null())
        .filter(projects::Column::PublishedAt.lt(project.published_at))
        .order_by_desc(projects::Column::PublishedAt)
        .one(db_pool)
        .await?;

    let next_model = projects::Entity::find()
        .filter(projects::Column::PublishedAt.is_not_null())
        .filter(projects::Column::PublishedAt.gt(project.published_at))
        .order_by_asc(projects::Column::PublishedAt)
        .one(db_pool)
        .await?;

    let prev = prev_model.map(ProjectAdjacent::from);
    let next = next_model.map(ProjectAdjacent::from);
    Ok(ProjectsAdjacentResponse { prev, next })
}
