use entities::{comments, likes, posts, shares, tags};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};

use crate::{
    dtos::post_dtos::{PostAdjacent, PostCollection, PostMetrics, PostsAdjacentResponse},
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
) -> Result<(Vec<PostCollection>, PaginationMeta), AppError> {
    let paginator = posts::Entity::find()
        .filter(posts::Column::PublishedAt.is_not_null())
        .order_by_desc(posts::Column::Id)
        .paginate(db_pool, 12);

    let (projects, meta) = PaginationMeta::paginate(&paginator, page).await?;
    Ok((
        projects
            .into_iter()
            .map(PostCollection::from)
            .collect::<Vec<PostCollection>>(),
        meta,
    ))
}

pub async fn get_pinned(db_pool: &DatabaseConnection) -> Result<Vec<PostCollection>, AppError> {
    let projects = posts::Entity::find()
        .filter(posts::Column::PublishedAt.is_not_null())
        .filter(posts::Column::IsPinned.eq(true))
        .order_by_desc(posts::Column::Id)
        .all(db_pool)
        .await?;

    Ok(projects
        .into_iter()
        .map(PostCollection::from)
        .collect::<Vec<PostCollection>>())
}

pub async fn find_by_slug_or_fail(
    db: &DatabaseConnection,
    slug: &str,
) -> Result<posts::Model, AppError> {
    posts::Entity::find()
        .filter(posts::Column::PublishedAt.is_not_null())
        .filter(posts::Column::Slug.eq(slug))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Post not found"))
}

pub async fn find_or_fail(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<posts::Model, AppError> {
    posts::Entity::find()
        .filter(posts::Column::PublishedAt.is_not_null())
        .filter(posts::Column::Id.eq(project_id))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Post not found"))
}

pub async fn get_metrics(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<PostMetrics, AppError> {
    let comments_count = comments::Entity::find()
        .filter(comments::Column::CommentableId.eq(project_id))
        .filter(comments::Column::CommentableType.eq(CommentableEnum::Post))
        .count(db_pool)
        .await?;

    let likes_count = likes::Entity::find()
        .filter(likes::Column::LikeableId.eq(project_id))
        .filter(likes::Column::LikeableType.eq(LikeableEnum::Post))
        .count(db_pool)
        .await?;

    let shares_count = shares::Entity::find()
        .filter(shares::Column::SharerId.eq(project_id))
        .filter(shares::Column::ShareableType.eq(ShareableEnum::Post))
        .count(db_pool)
        .await?;

    Ok(PostMetrics {
        project_id,
        comments_count,
        likes_count,
        shares_count,
    })
}

pub async fn get_tags(
    post_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<Vec<tags::Model>, AppError> {
    let post = find_or_fail(post_id, db_pool).await?;
    post.find_related(tags::Entity)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}

pub async fn adjacent(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<PostsAdjacentResponse, AppError> {
    let project = find_or_fail(project_id, db_pool).await?;

    let prev_model = posts::Entity::find()
        .filter(posts::Column::PublishedAt.is_not_null())
        .filter(posts::Column::PublishedAt.lt(project.published_at))
        .order_by_desc(posts::Column::PublishedAt)
        .one(db_pool)
        .await?;

    let next_model = posts::Entity::find()
        .filter(posts::Column::PublishedAt.is_not_null())
        .filter(posts::Column::PublishedAt.gt(project.published_at))
        .order_by_asc(posts::Column::PublishedAt)
        .one(db_pool)
        .await?;

    let prev = prev_model.map(PostAdjacent::from);
    let next = next_model.map(PostAdjacent::from);
    Ok(PostsAdjacentResponse { prev, next })
}
