use actix_web::{
    Responder,
    web::{Data, Path, Query},
};

use crate::helpers::pagination_meta::PaginationParams;
use crate::{
    dtos::project_dtos::ProjectResponse,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::project_repositories,
};

pub async fn index(
    app_state: Data<AppState>,
    query: Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let (project_response, meta) =
        project_repositories::get_paginate_projects(&app_state.db_pool, query.page.unwrap_or(1))
            .await?;

    Ok(ApiResponse::ok_with_pagination(
        "Projects retrieved successfully",
        project_response,
        meta,
    ))
}

pub async fn pinned(app_state: Data<AppState>) -> Result<impl Responder, AppError> {
    Ok(ApiResponse::ok(
        "Projects pinned retrieved successfully",
        Some(project_repositories::get_pinned_projects(&app_state.db_pool).await?),
    ))
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

pub async fn metrics(
    project_id: Path<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    Ok(ApiResponse::ok(
        "Project metrics retrieved successfully",
        Some(project_repositories::get_metrics(project_id.into_inner(), &app_state.db_pool).await?),
    ))
}

pub async fn adjacent(
    project_id: Path<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    Ok(ApiResponse::ok(
        "Project adjacent retrieved succesfully",
        Some(project_repositories::adjacent(project_id.into_inner(), &app_state.db_pool).await?),
    ))
}
