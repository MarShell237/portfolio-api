use entities::{comments, likes, posts, shares, tags};
use sea_orm::{
    ActiveEnum, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
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
        .filter(posts::COLUMN.published_at.is_not_null())
        .order_by_desc(posts::COLUMN.id)
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
        .filter(posts::COLUMN.published_at.is_not_null())
        .filter(posts::COLUMN.is_pinned.eq(true))
        .order_by_desc(posts::COLUMN.id)
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
        .filter(posts::COLUMN.published_at.is_not_null())
        .filter(posts::COLUMN.slug.eq(slug))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Post not found"))
}

pub async fn find_or_fail(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<posts::Model, AppError> {
    posts::Entity::find()
        .filter(posts::COLUMN.published_at.is_not_null())
        .filter(posts::COLUMN.id.eq(project_id))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Post not found"))
}

pub async fn get_metrics(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<PostMetrics, AppError> {
    let comments_count = comments::Entity::find()
        .filter(comments::COLUMN.commentable_id.eq(project_id))
        .filter(
            comments::COLUMN
                .commentable_type
                .eq(CommentableEnum::Post.to_value()),
        )
        .count(db_pool)
        .await?;

    let likes_count = likes::Entity::find()
        .filter(likes::COLUMN.likeable_id.eq(project_id))
        .filter(
            likes::COLUMN
                .likeable_type
                .eq(LikeableEnum::Post.to_value()),
        )
        .count(db_pool)
        .await?;

    let shares_count = shares::Entity::find()
        .filter(shares::COLUMN.sharer_id.eq(project_id))
        .filter(
            shares::COLUMN
                .shareable_type
                .eq(ShareableEnum::Post.to_value()),
        )
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
    post_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<PostsAdjacentResponse, AppError> {
    let post = find_or_fail(post_id, db_pool).await?;
    let published_at = match post.published_at {
        Some(date) => date,
        None => {
            return Ok(PostsAdjacentResponse {
                prev: None,
                next: None,
            });
        }
    };
    let prev_model = posts::Entity::find()
        .filter(posts::COLUMN.published_at.is_not_null())
        .filter(posts::COLUMN.published_at.lt(published_at))
        .order_by_desc(posts::COLUMN.published_at)
        .one(db_pool)
        .await?;

    let next_model = posts::Entity::find()
        .filter(posts::COLUMN.published_at.is_not_null())
        .filter(posts::COLUMN.published_at.gt(published_at))
        .order_by_asc(posts::COLUMN.published_at)
        .one(db_pool)
        .await?;

    let prev = prev_model.map(PostAdjacent::from);
    let next = next_model.map(PostAdjacent::from);
    Ok(PostsAdjacentResponse { prev, next })
}
