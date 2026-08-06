use crate::dtos::tag_dtos::TagResponse;
use crate::helpers::{api_response::ApiResponse, app_state::AppState};
use actix_web::http::StatusCode;
use actix_web::web::Path;
use actix_web::{Responder, web};
use entities::tags::{Column as TagColumn, Entity as Tag};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn index(app_state: web::Data<AppState>) -> impl Responder {
    let tags = Tag::find().all(&app_state.db_pool).await.unwrap();
    let tag_response: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();
    ApiResponse::ok("Tags retrieved successfully", Some(tag_response))
}

pub async fn show(slug: Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    match Tag::find()
        .filter(TagColumn::Slug.eq(slug.into_inner()))
        .one(&app_state.db_pool)
        .await
    {
        Ok(Some(tag)) => ApiResponse::ok(
            "Tag details retrieved successfully",
            Some(TagResponse::from(tag)),
        ),
        Ok(None) => ApiResponse::error(
            StatusCode::NOT_FOUND,
            "Failled to retieved Tag details",
            None,
        ),
        Err(_) => ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Database error", None),
    }
}
