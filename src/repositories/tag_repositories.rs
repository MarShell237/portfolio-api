use crate::errors::AppError;
use entities::{post_tag, posts, project_tag, projects, tags};
use migrations::SimpleExpr;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, Order, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
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
        .filter(tags::Column::Slug.eq(slug))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Tag not found"))
}

pub async fn find_by_type_with_count(
    db_pool: &DatabaseConnection,
    tag_type: &str,
) -> Result<Vec<tags::Model>, AppError> {
    let mut query = tags::Entity::find();

    match tag_type {
        "posts" => {
            query = query
                .join(JoinType::InnerJoin, tags::Relation::PostTag.def())
                .group_by(tags::Column::Id)
                .order_by_desc(SimpleExpr::from(post_tag::Column::PostId.count()));
        }
        "projects" => {
            query = query
                .join(JoinType::InnerJoin, tags::Relation::ProjectTag.def())
                .group_by(tags::Column::Id)
                .order_by_desc(SimpleExpr::from(project_tag::Column::ProjectId.count()));
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
) -> Result<Vec<projects::Model>, AppError> {
    find_by_slug_or_fail(db_pool, slug).await?;

    projects::Entity::find()
        .join(
            JoinType::InnerJoin,
            entities::project_tag::Relation::Projects.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            entities::project_tag::Relation::Tags.def(),
        )
        .filter(tags::Column::Slug.eq(slug))
        .filter(projects::Column::PublishedAt.is_not_null())
        .order_by(projects::Column::Id, Order::Desc)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}

pub async fn find_posts_by_tag_slug(
    db_pool: &DatabaseConnection,
    slug: &str,
) -> Result<Vec<posts::Model>, AppError> {
    find_by_slug_or_fail(db_pool, slug).await?;

    posts::Entity::find()
        .join(
            JoinType::InnerJoin,
            entities::post_tag::Relation::Posts.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            entities::post_tag::Relation::Tags.def(),
        )
        .filter(tags::Column::Slug.eq(slug))
        .filter(posts::Column::PublishedAt.is_not_null())
        .order_by(posts::Column::Id, Order::Desc)
        .all(db_pool)
        .await
        .map_err(AppError::from)
}
