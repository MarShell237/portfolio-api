use crate::{
    dtos::{
        post_dtos::PostCollection,
        project_dtos::ProjectCollection,
        tag_dtos::{TagQuery, TagResponse},
    },
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::tag_repositories,
};
use actix_web::{
    Responder,
    web::{Data, Path, Query},
};

pub async fn index(
    app_state: Data<AppState>,
    query: Query<TagQuery>,
) -> Result<impl Responder, AppError> {
    let tag_type = query.r#type.as_deref().ok_or_else(|| {
        AppError::bad_request(
            "Missing required 'type' query parameter. Expected 'posts' or 'projects'",
        )
    })?;

    let tags = tag_repositories::find_by_type_with_count(&app_state.db_pool, tag_type).await?;

    let tag_responses: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();

    Ok(ApiResponse::ok(
        "Tags retrieved successfully",
        Some(tag_responses),
    ))
}

pub async fn show(
    slug: Path<String>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    Ok(ApiResponse::ok(
        "Tag details retrieved successfully",
        Some(TagResponse::from(
            tag_repositories::find_by_slug_or_fail(&app_state.db_pool, &slug).await?,
        )),
    ))
}

pub async fn get_projects(
    slug: Path<String>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let projects = tag_repositories::find_projects_by_tag_slug(&app_state.db_pool, &slug).await?;
    let responses: Vec<ProjectCollection> =
        projects.into_iter().map(ProjectCollection::from).collect();

    Ok(ApiResponse::ok(
        "Projects retrieved successfully",
        Some(responses),
    ))
}

pub async fn get_posts(
    slug: Path<String>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let posts = tag_repositories::find_posts_by_tag_slug(&app_state.db_pool, &slug).await?;
    let responses: Vec<PostCollection> = posts.into_iter().map(PostCollection::from).collect();

    Ok(ApiResponse::ok(
        "Posts retrieved successfully",
        Some(responses),
    ))
}
