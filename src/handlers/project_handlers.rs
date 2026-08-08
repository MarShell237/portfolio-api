use actix_web::{
    Responder,
    web::{Data, Query},
};
use entities::projects::{Column, Entity as Project};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{
    dtos::project_dtos::ProjectResponse,
    helpers::{
        api_response::ApiResponse,
        app_state::AppState,
        pagination_meta::{PaginationMeta, PaginationParams},
    },
};

pub async fn index(app_state: Data<AppState>, query: Query<PaginationParams>) -> impl Responder {
    let page = query.page.unwrap_or(1);

    let paginator = Project::find()
        .filter(Column::PublishedAt.is_not_null())
        .order_by_desc(Column::Id)
        .paginate(&app_state.db_pool, 12);

    match PaginationMeta::paginate(&paginator, page).await {
        Ok((projects, meta)) => {
            let projects_response: Vec<ProjectResponse> =
                projects.into_iter().map(ProjectResponse::from).collect();
            ApiResponse::ok_with_pagination(
                "Projects retrieved successfully",
                projects_response,
                meta,
            )
        }
        Err(_) => ApiResponse::internal_server_error("Failed to fetch projects"),
    }
}
