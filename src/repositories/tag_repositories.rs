use crate::errors::AppError;
use entities::{
    post_tag::Column as PostTagColumn,
    posts::{Column as PostColumn, Entity as Post, Model as PostModel},
    project_tag::Column as ProjectTagColumn,
    projects::{Column as ProjectColumn, Entity as Project, Model as ProjectModel},
    tags::{Column as TagColumn, Entity as Tag, Model as TagModel, Relation as TagRelation},
};
use migrations::SimpleExpr;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, Order, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};

pub async fn find_by_slug_or_fail(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<TagModel, AppError> {
    Tag::find()
        .filter(TagColumn::Slug.eq(slug))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Tag not found"))
}

pub async fn find_by_type_with_count(
    db_pool: &DatabaseConnection,
    tag_type: &str,
) -> Result<Vec<TagModel>, AppError> {
    let mut query = Tag::find();

    match tag_type {
        "posts" => {
            query = query
                .join(JoinType::InnerJoin, TagRelation::PostTag.def())
                .group_by(TagColumn::Id)
                .order_by_desc(SimpleExpr::from(PostTagColumn::PostId.count()));
        }
        "projects" => {
            query = query
                .join(JoinType::InnerJoin, TagRelation::ProjectTag.def())
                .group_by(TagColumn::Id)
                .order_by_desc(SimpleExpr::from(ProjectTagColumn::ProjectId.count()));
        }
        _ => {
            return Err(AppError::not_found(
                "Invalid tag type. Must be 'posts' or 'projects'",
            ));
        }
    }

    query.all(db_pool).await.map_err(AppError::from)
}

pub async fn find_projects_by_tag_slug(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<Vec<ProjectModel>, AppError> {
    find_by_slug_or_fail(db_pool, slug).await?;

    Project::find()
        .join(
            JoinType::InnerJoin,
            entities::project_tag::Relation::Projects.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            entities::project_tag::Relation::Tags.def(),
        )
        .filter(TagColumn::Slug.eq(slug))
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .order_by(ProjectColumn::Id, Order::Desc)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}

pub async fn find_posts_by_tag_slug(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<Vec<PostModel>, AppError> {
    find_by_slug_or_fail(db_pool, slug).await?;

    Post::find()
        .join(
            JoinType::InnerJoin,
            entities::post_tag::Relation::Posts.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            entities::post_tag::Relation::Tags.def(),
        )
        .filter(TagColumn::Slug.eq(slug))
        .filter(PostColumn::PublishedAt.is_not_null())
        .order_by(PostColumn::Id, Order::Desc)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}
