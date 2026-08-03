use crate::dtos::tag_dtos::TagResponse;
use crate::helpers::app_state::AppState;
use actix_web::{HttpResponse, Responder, web};
use entities::tags::Entity as Tag;
use sea_orm::EntityTrait;

pub async fn index(app_state: web::Data<AppState>) -> impl Responder {
    let tags = Tag::find().all(&app_state.db_pool).await.unwrap();
    let tag_response: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();
    let tag_json = serde_json::to_string(&tag_response).unwrap();
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(tag_json)
}
