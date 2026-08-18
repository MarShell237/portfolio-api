use entities::{
    comments::{Column as CommentColumn, Entity as Comment},
    likes::{Column as LikeColumn, Entity as Like},
    posts::{Column as PostColumn, Entity as Post, Model as PostModel},
    shares::{Column as ShareColumn, Entity as Share},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
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
    let paginator = Post::find()
        .filter(PostColumn::PublishedAt.is_not_null())
        .order_by_desc(PostColumn::Id)
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
    let projects = Post::find()
        .filter(PostColumn::PublishedAt.is_not_null())
        .filter(PostColumn::IsPinned.eq(true))
        .order_by_desc(PostColumn::Id)
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
) -> Result<PostModel, AppError> {
    Post::find()
        .filter(PostColumn::PublishedAt.is_not_null())
        .filter(PostColumn::Slug.eq(slug))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Post not found"))
}

pub async fn find_or_fail(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<PostModel, AppError> {
    Post::find()
        .filter(PostColumn::PublishedAt.is_not_null())
        .filter(PostColumn::Id.eq(project_id))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Post not found"))
}

pub async fn get_metrics(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<PostMetrics, AppError> {
    let comments_count = Comment::find()
        .filter(CommentColumn::CommentableId.eq(project_id))
        .filter(CommentColumn::CommentableType.eq(CommentableEnum::Post))
        .count(db_pool)
        .await?;

    let likes_count = Like::find()
        .filter(LikeColumn::LikeableId.eq(project_id))
        .filter(LikeColumn::LikeableType.eq(LikeableEnum::Post))
        .count(db_pool)
        .await?;

    let shares_count = Share::find()
        .filter(ShareColumn::SharerId.eq(project_id))
        .filter(ShareColumn::ShareableType.eq(ShareableEnum::Post))
        .count(db_pool)
        .await?;

    Ok(PostMetrics {
        project_id,
        comments_count,
        likes_count,
        shares_count,
    })
}

pub async fn adjacent(
    project_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<PostsAdjacentResponse, AppError> {
    let project = find_or_fail(project_id, db_pool).await?;

    let prev_model = Post::find()
        .filter(PostColumn::PublishedAt.is_not_null())
        .filter(PostColumn::PublishedAt.lt(project.published_at))
        .order_by_desc(PostColumn::PublishedAt)
        .one(db_pool)
        .await?;

    let next_model = Post::find()
        .filter(PostColumn::PublishedAt.is_not_null())
        .filter(PostColumn::PublishedAt.gt(project.published_at))
        .order_by_asc(PostColumn::PublishedAt)
        .one(db_pool)
        .await?;

    let prev = prev_model.map(PostAdjacent::from);
    let next = next_model.map(PostAdjacent::from);
    Ok(PostsAdjacentResponse { prev, next })
}
