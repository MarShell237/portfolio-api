use actix_web::{
    Responder,
    web::{Data, Path, ReqData},
};

use crate::{
    enums::likeable_enum::LikeableEnum,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::like_repositories,
};

pub async fn like_or_unlike(
    path: Path<(String, i64)>,
    user_id: ReqData<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let (likeable_type, likeable_id) = path.into_inner();
    LikeableEnum::validate_likeable(likeable_type.as_str(), likeable_id, &app_state.db_pool)
        .await?;
    let message = if like_repositories::already_liked(
        *user_id,
        likeable_type.clone(),
        likeable_id,
        &app_state.db_pool,
    )
    .await?
    {
        like_repositories::unlike(*user_id, likeable_type, likeable_id, &app_state.db_pool).await?;
        "like retirer avec succes"
    } else {
        like_repositories::like(*user_id, likeable_type, likeable_id, &app_state.db_pool).await?;
        "like ajouter avec succes"
    };
    Ok(ApiResponse::ok(message, None::<()>))
}
