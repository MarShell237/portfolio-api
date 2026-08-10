use actix_web::{
    Responder,
    web::{Data, Path, Query},
};
use entities::projects::{Column as ProjectColumn, Entity as Project};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{
    dtos::project_dtos::{ProjectCollection, ProjectResponse},
    errors::AppError,
    helpers::{
        api_response::ApiResponse,
        app_state::AppState,
        pagination_meta::{PaginationMeta, PaginationParams},
    },
    repositories::project_repositories,
};

pub async fn index(app_state: Data<AppState>, query: Query<PaginationParams>) -> impl Responder {
    let page = query.page.unwrap_or(1);

    let paginator = Project::find()
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .order_by_desc(ProjectColumn::Id)
        .paginate(&app_state.db_pool, 12);

    match PaginationMeta::paginate(&paginator, page).await {
        Ok((projects, meta)) => {
            let projects_response: Vec<ProjectCollection> =
                projects.into_iter().map(ProjectCollection::from).collect();
            ApiResponse::ok_with_pagination(
                "Projects retrieved successfully",
                projects_response,
                meta,
            )
        }
        Err(_) => ApiResponse::internal_server_error("Failed to fetch projects"),
    }
}

pub async fn pinned(app_state: Data<AppState>) -> impl Responder {
    let projects = Project::find()
        .filter(ProjectColumn::PublishedAt.is_not_null())
        .filter(ProjectColumn::IsPinned.eq(true))
        .order_by_desc(ProjectColumn::Id)
        .all(&app_state.db_pool)
        .await
        .unwrap();

    let projects_response: Vec<ProjectCollection> =
        projects.into_iter().map(ProjectCollection::from).collect();
    ApiResponse::ok(
        "Projects pinned retrieved successfully",
        Some(projects_response),
    )
}

pub async fn show(
    slug: Path<String>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let project = project_repositories::find_by_slug_or_fail(&app_state.db_pool, &slug).await?;

    Ok(ApiResponse::ok(
        "Project details retrieved successfully",
        Some(ProjectResponse::from(project)),
    ))
}
