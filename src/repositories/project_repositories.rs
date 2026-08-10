use entities::projects::{Column as ProjectColumn, Entity as Project, Model as ProjectModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::errors::AppError;

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
