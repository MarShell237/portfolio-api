use entities::{comments, likes};
use sea_orm::{
    ActiveEnum, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter,
};

use crate::{
    dtos::responses::comment_meta::CommentMeta, enums::likeable_enum::LikeableEnum,
    errors::AppError, helpers::pagination_meta::PaginationMeta, repositories::user_repositories,
};

pub async fn find_or_fail(
    comment_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<comments::Model, AppError> {
    comments::Entity::find_by_id(comment_id)
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Comment not found"))
}

pub async fn get_comments_of_commentable(
    commentable_type: String,
    commentable_id: i64,
    page: u64,
    db_pool: &DatabaseConnection,
) -> Result<(Vec<comments::Model>, PaginationMeta), AppError> {
    let paginator = comments::Entity::find()
        .filter(comments::COLUMN.commentable_type.eq(commentable_type))
        .filter(comments::COLUMN.commentable_id.eq(commentable_id))
        .paginate(db_pool, 5);
    Ok(PaginationMeta::paginate(&paginator, page).await?)
}

pub async fn get_comment_meta(
    comment_id: i64,
    user_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<CommentMeta, AppError> {
    let likes_count = likes::Entity::find()
        .filter(likes::COLUMN.likeable_id.eq(comment_id))
        .filter(
            likes::COLUMN
                .likeable_type
                .eq(LikeableEnum::Comment.to_value()),
        )
        .count(db_pool)
        .await?;
    let comments_count = comments::Entity::find()
        .filter(comments::COLUMN.parent_id.eq(comment_id))
        .count(db_pool)
        .await?;
    let is_liked = likes::Entity::find()
        .filter(likes::COLUMN.likeable_id.eq(comment_id))
        .filter(
            likes::COLUMN
                .likeable_type
                .eq(LikeableEnum::Comment.to_value()),
        )
        .filter(likes::COLUMN.liker_id.eq(user_id))
        .one(db_pool)
        .await?
        .is_some();
    let admin = user_repositories::get_first_admin(db_pool).await?;
    let is_liked_by_admin = likes::Entity::find()
        .filter(likes::COLUMN.likeable_id.eq(comment_id))
        .filter(
            likes::COLUMN
                .likeable_type
                .eq(LikeableEnum::Comment.to_value()),
        )
        .filter(likes::COLUMN.liker_id.eq(admin.id))
        .one(db_pool)
        .await?
        .is_some();
    let has_admin_reply = comments::Entity::find()
        .filter(comments::COLUMN.parent_id.eq(comment_id))
        .filter(comments::COLUMN.commenter_id.eq(admin.id))
        .one(db_pool)
        .await?
        .is_some();
    Ok(CommentMeta {
        likes_count,
        comments_count,
        is_liked,
        is_liked_by_admin,
        has_admin_reply,
    })
}

pub async fn create(
    user_id: i64,
    commentable_type: String,
    commentable_id: i64,
    content: String,
    db_pool: &DatabaseConnection,
) -> Result<comments::Model, AppError> {
    comments::ActiveModel {
        commenter_id: Set(user_id),
        commentable_type: Set(commentable_type),
        commentable_id: Set(commentable_id),
        content: Set(content),
        ..Default::default()
    }
    .insert(db_pool)
    .await
    .map_err(AppError::from)
}
