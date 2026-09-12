use actix_web::{
    Responder,
    web::{Data, Path, Query},
};
use entities::projects;

use crate::{
    dtos::project_dtos::ProjectResponse,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::project_repositories,
};
use crate::{dtos::tag_dtos::TagResponse, helpers::pagination_meta::PaginationParams};

pub async fn index(
    app_state: Data<AppState>,
    query: Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let (project_response, meta) =
        project_repositories::get_paginate(&app_state.db_pool, query.page.unwrap_or(1)).await?;

    Ok(ApiResponse::ok_with_pagination(
        "Projects retrieved successfully",
        project_response,
        meta,
    ))
}

pub async fn pinned(app_state: Data<AppState>) -> Result<impl Responder, AppError> {
    Ok(ApiResponse::ok(
        "Projects pinned retrieved successfully",
        Some(project_repositories::get_pinned(&app_state.db_pool).await?),
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

pub async fn get_tags(
    project_id: Path<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let tags = project_repositories::get_tags(project_id.into_inner(), &app_state.db_pool).await?;
    let tags_response: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();
    Ok(ApiResponse::ok(
        "Project tags retrieved succesfully",
        Some(tags_response),
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
