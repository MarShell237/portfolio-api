use crate::{
    dtos::tag_dtos::TagResponse,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::tag_repositories,
};
use actix_web::{
    Responder,
    web::{Data, Path},
};
use entities::tags::Entity as Tag;
use sea_orm::EntityTrait;

pub async fn index(app_state: Data<AppState>) -> impl Responder {
    let tags = Tag::find().all(&app_state.db_pool).await.unwrap();
    let tag_response: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();
    ApiResponse::ok("Tags retrieved successfully", Some(tag_response))
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
