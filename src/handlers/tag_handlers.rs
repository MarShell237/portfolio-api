use crate::dtos::tag_dtos::TagResponse;
use crate::helpers::{api_response::ApiResponse, app_state::AppState};
use actix_web::{
    Responder,
    web::{Data, Path},
};
use entities::tags::{Column as TagColumn, Entity as Tag};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn index(app_state: Data<AppState>) -> impl Responder {
    let tags = Tag::find().all(&app_state.db_pool).await.unwrap();
    let tag_response: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();
    ApiResponse::ok("Tags retrieved successfully", Some(tag_response))
}

pub async fn show(slug: Path<String>, app_state: Data<AppState>) -> impl Responder {
    match Tag::find()
        .filter(TagColumn::Slug.eq(slug.into_inner()))
        .one(&app_state.db_pool)
        .await
    {
        Ok(Some(tag)) => ApiResponse::ok(
            "Tag details retrieved successfully",
            Some(TagResponse::from(tag)),
        ),
        Ok(None) => ApiResponse::not_found("Failled to retieved Tag details"),
        Err(_) => ApiResponse::internal_server_error("Database error"),
    }
}
