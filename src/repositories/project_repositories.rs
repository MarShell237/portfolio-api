use entities::{
    comments::{Column as CommentColumn, Entity as Comment},
    likes::{Column as LikeColumn, Entity as Like},
    projects::{Column as ProjectColumn, Entity as Project, Model as ProjectModel},
    shares::{Column as ShareColumn, Entity as Share},
};
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
    let paginator = Project::find()
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .order_by_desc(ProjectColumn::Id)
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
    let projects = Project::find()
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .filter(ProjectColumn::IsPinned.eq(true))
        .order_by_desc(ProjectColumn::Id)
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
) -> Result<ProjectModel, AppError> {
    Project::find()
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .filter(ProjectColumn::Slug.eq(slug))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Project not found"))
}

pub async fn find_or_fail(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<ProjectModel, AppError> {
    Project::find()
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .filter(ProjectColumn::Id.eq(project_id))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Project not found"))
}

pub async fn get_metrics(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<ProjectMetrics, AppError> {
    let comments_count = Comment::find()
        .filter(CommentColumn::CommentableId.eq(project_id))
        .filter(CommentColumn::CommentableType.eq(CommentableEnum::Project))
        .count(db_pool)
        .await?;

    let likes_count = Like::find()
        .filter(LikeColumn::LikeableId.eq(project_id))
        .filter(LikeColumn::LikeableType.eq(LikeableEnum::Project))
        .count(db_pool)
        .await?;

    let shares_count = Share::find()
        .filter(ShareColumn::SharerId.eq(project_id))
        .filter(ShareColumn::ShareableType.eq(ShareableEnum::Project))
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

    let prev_model = Project::find()
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .filter(ProjectColumn::PublishedAt.lt(project.published_at))
        .order_by_desc(ProjectColumn::PublishedAt)
        .one(db_pool)
        .await?;

    let next_model = Project::find()
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .filter(ProjectColumn::PublishedAt.gt(project.published_at))
        .order_by_asc(ProjectColumn::PublishedAt)
        .one(db_pool)
        .await?;

    let prev = prev_model.map(ProjectAdjacent::from);
    let next = next_model.map(ProjectAdjacent::from);
    Ok(ProjectsAdjacentResponse { prev, next })
}
