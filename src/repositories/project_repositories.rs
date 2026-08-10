use entities::{
    comments::{Column as CommentColumn, Entity as Comment},
    likes::{Column as LikeColumn, Entity as Like},
    projects::{Column as ProjectColumn, Entity as Project, Model as ProjectModel},
    shares::{Column as ShareColumn, Entity as Share},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

use crate::{
    dtos::project_dtos::ProjectMetrics,
    enums::{
        commentable_enum::CommentableEnum, likeable_enum::LikeableEnum,
        shareable_enum::ShareableEnum,
    },
    errors::AppError,
};

pub async fn find_by_slug_or_fail(
    db: &DatabaseConnection,
    slug: &str,
) -> Result<ProjectModel, AppError> {
    let project = Project::find()
        .filter(ProjectColumn::Slug.eq(slug))
        .one(db)
        .await?;

    project.ok_or_else(|| AppError::not_found("Project not found"))
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
