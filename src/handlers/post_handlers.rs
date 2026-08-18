use actix_web::{
    Responder,
    web::{Data, Path, Query},
};

use crate::helpers::pagination_meta::PaginationParams;
use crate::{
    dtos::post_dtos::PostResponse,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::post_repositories,
};

pub async fn index(
    app_state: Data<AppState>,
    query: Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let (post_response, meta) =
        post_repositories::get_paginate(&app_state.db_pool, query.page.unwrap_or(1)).await?;

    Ok(ApiResponse::ok_with_pagination(
        "Posts retrieved successfully",
        post_response,
        meta,
    ))
}

pub async fn pinned(app_state: Data<AppState>) -> Result<impl Responder, AppError> {
    Ok(ApiResponse::ok(
        "Posts pinned retrieved successfully",
        Some(post_repositories::get_pinned(&app_state.db_pool).await?),
    ))
}

pub async fn show(
    slug: Path<String>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let post = post_repositories::find_by_slug_or_fail(&app_state.db_pool, &slug).await?;

    Ok(ApiResponse::ok(
        "Post details retrieved successfully",
        Some(PostResponse::from(post)),
    ))
}

pub async fn metrics(
    post_id: Path<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    Ok(ApiResponse::ok(
        "Post metrics retrieved successfully",
        Some(post_repositories::get_metrics(post_id.into_inner(), &app_state.db_pool).await?),
    ))
}

pub async fn adjacent(
    post_id: Path<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    Ok(ApiResponse::ok(
        "Post adjacent retrieved succesfully",
        Some(post_repositories::adjacent(post_id.into_inner(), &app_state.db_pool).await?),
    ))
}
